import 'dart:convert';
import 'dart:math';
import 'dart:core';
import 'package:homeflow/models/device.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:mqtt_client/mqtt_server_client.dart';
import 'package:homeflow/shared/constants.dart';
import 'dart:async';

import 'package:provider/provider.dart';

class MqttService extends ChangeNotifier implements ReassembleHandler {
  MqttClient mqttClient;

  final String userUid;

  final Future<String> Function([bool]) getToken;

  Future<MqttClient> connect() async {
    final clientShortId = "mobile_${getRandomShortString()}";

    mqttClient = MqttServerClient.withPort(MQTT_HOST, clientShortId, MQTT_PORT);
    mqttClient.autoReconnect = true;

    final token = getToken();
    Completer<MqttClient> completer = Completer<MqttClient>();
//    mqttClient.logging(on: true);
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
        .withClientIdentifier(clientShortId)
        .authenticateAs(userUid, await token)
        .keepAliveFor(60)
        .startClean();
    mqttClient.connectionMessage = connMessage;

    mqttClient.connect().catchError((e) {
      print("MQTT Exception $e");
      mqttClient.disconnect();
    });
    return await completer.future;
  }

  MqttService({@required this.userUid, @required this.getToken});

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

    return await completer.future;
  }

  @override
  void dispose() {
    mqttClient.disconnect();
    mqttClient.autoReconnect = false;
    print("Disconnected from MQTT due to dispose");
    super.dispose();
  }

  @override
  void reassemble() {
    mqttClient.disconnect();
    mqttClient.autoReconnect = false;
    print("Disconnected from MQTT due to reassemble");
  }
}
