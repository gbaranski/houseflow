import 'dart:async';
import 'dart:convert';
import 'dart:core';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:mqtt_client/mqtt_server_client.dart';

enum ConnectionStatus {
  connected,
  disconnected,
  failed,
  not_attempted,
}

const int maxConnectionAttempts = 4;

class MqttService extends ChangeNotifier {
  MqttServerClient mqttClient;
  int connectionAttempts = 0;

  ConnectionStatus _connectionStatus = ConnectionStatus.not_attempted;

  ConnectionStatus get connectionStatus => _connectionStatus;

  void resetConnectionStatus() {
    _connectionStatus = ConnectionStatus.not_attempted;
  }

  Future<void> connect(
      {@required String userUid, @required Future<String> token}) async {
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
      _connectionStatus = ConnectionStatus.disconnected;
      notifyListeners();
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
      _connectionStatus = ConnectionStatus.connected;
    else {
      _connectionStatus = ConnectionStatus.failed;
      FirebaseService.crashlytics.recordError(
          "mqtt_connection_failed", StackTrace.current, information: [
        DiagnosticsNode.message('return_code: ${result.returnCode.toString()}')
      ]);
      print(
          "Connection failed with MQTT, return code: ${result.disconnectionOrigin}");
    }

    notifyListeners();
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

  @override
  void dispose() {
    disconnect("dispose");
    print("Disposing MQTT Service class");
    super.dispose();
  }

  void disconnect(String reason) {
    mqttClient?.disconnect();
    _connectionStatus = ConnectionStatus.disconnected;
    print("Disconnected due to $reason");
  }
}
