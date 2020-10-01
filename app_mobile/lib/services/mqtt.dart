import 'dart:convert';
import 'dart:math';
import 'dart:core';
import 'package:homeflow/models/device.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:mqtt_client/mqtt_server_client.dart';
import 'package:homeflow/shared/constants.dart';
import 'package:homeflow/models/user.dart';
import 'dart:async';

class MqttService extends ChangeNotifier {
  MqttClient mqttClient;

  MqttService({@required String userUid, @required String token}) {
    print("MQTT Service constructor");

    Completer<MqttClient> completer = Completer();
    mqttClient = MqttServerClient.withPort(MQTT_HOST, userUid, MQTT_PORT);
    mqttClient.logging(on: true);
    mqttClient.onConnected = () {
      print("Connected to MQTT");
      completer.complete(mqttClient);
    };
    mqttClient.onDisconnected = () {
      print("Disconnected from MQTT");
    };
    mqttClient.onSubscribeFail = (topic) {
      print("Failed subscribe to $topic");
    };

    final connMessage = MqttConnectMessage()
        .withClientIdentifier(userUid)
        .authenticateAs(userUid, token)
        .keepAliveFor(60)
        .startClean();
    mqttClient.connectionMessage = connMessage;

    mqttClient.connect().catchError((e) {
      print("MQTT Exception $e");
      mqttClient.disconnect();
    });
  }

  static String getRandomShortString() {
    final randomNum = Random();
    final List<int> randomInts =
        List<int>.generate(8, (i) => randomNum.nextInt(256));
    return base64UrlEncode(randomInts);
  }

  Future sendMessage(
      {RequestTopic topic, MqttQos qos, Map<String, dynamic> data}) async {
    final randomShortString = getRandomShortString();
    final completer = Completer();

    StreamSubscription streamSubscription;

    streamSubscription =
        mqttClient.updates.listen((List<MqttReceivedMessage<MqttMessage>> c) {
      final MqttPublishMessage message = c[0].payload;
      final payload =
          MqttPublishPayload.bytesToStringAsString(message.payload.message);

      final Map<String, dynamic> responseData = jsonDecode(payload);
      print('Received message:$payload from topic: ${c[0].topic}>');
      if (responseData['correlationData'] == randomShortString) {
        streamSubscription.cancel();
        completer.complete();
      }
    });

    mqttClient.subscribe(topic.response, MqttQos.atLeastOnce);

    final Map<String, dynamic> json = {
      'data': data,
      'correlationData': randomShortString
    };
    final String stringifiedData = jsonEncode(json);
    final builder = MqttClientPayloadBuilder();
    builder.addString(stringifiedData);
    mqttClient.publishMessage(topic.request, qos, builder.payload,
        retain: false);
    print("Published message");

    return completer.future;
  }

  void _onError(
      dynamic data, FirebaseUser firebaseUser, auth.User currentUser) async {
    print("Error $data}");
    await Future.delayed(Duration(milliseconds: 1000));
    print("Reconnecting!");
  }
}
