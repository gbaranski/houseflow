import 'dart:async';
import 'dart:convert';
import 'dart:core';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:mqtt_client/mqtt_server_client.dart';
import 'package:provider/provider.dart';

enum ConnectionStatus {
  connected,
  disconnected,
  failed,
  attempts_exceeded,
  not_attempted,
}

const int maxConnectionAttempts = 5;

class MqttService extends ChangeNotifier implements ReassembleHandler {
  MqttServerClient mqttClient;
  int connectionAttempts = 0;

  final StreamController<ConnectionStatus> streamController =
      StreamController();

  @override
  void dispose() {
    super.dispose();
    print("Disposing MQTT Service class");
    streamController.close();
    disconnect("dispose");
  }

  Future<void> connect(
      {@required String userUid, @required Future<String> token}) async {
    if (connectionAttempts > maxConnectionAttempts)
      return streamController.add(ConnectionStatus.attempts_exceeded);

    final clientShortId = "mobile_${getRandomShortString()}";
    mqttClient = MqttServerClient.withPort(MQTT_HOST, clientShortId, MQTT_PORT);
    mqttClient.autoReconnect = false;
    mqttClient.resubscribeOnAutoReconnect = false;
    mqttClient.secure = true;
    // mqttClient.logging(on: true);

    // Security context
    final context = new SecurityContext();
    context.setTrustedCertificatesBytes(utf8.encode(ROOT_CA));

    mqttClient.onBadCertificate = (dynamic a) {
      print("Wrong MQTT certificate");
      print("a: $a");
      return false;
    };

    mqttClient.onConnected = () {
      print("Connected to MQTT");
    };
    mqttClient.onDisconnected = () {
      print("Disconnected from MQTT");
      if (!streamController.isClosed)
        streamController.add(ConnectionStatus.disconnected);
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
    final result = await mqttClient.connect();
    connectionAttempts += 1;

    if (result.state == MqttConnectionState.connected)
      streamController.add(ConnectionStatus.connected);
    else
      streamController.add(ConnectionStatus.failed);
  }

  Future sendMessage(
      {DeviceTopic topic, MqttQos qos, Map<String, dynamic> data}) async {
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

  void disconnect(String reason) {
    mqttClient?.disconnect();
    print("Disconnected due to $reason");
  }

  @override
  void reassemble() {
    disconnect("ressemble");
  }
}
