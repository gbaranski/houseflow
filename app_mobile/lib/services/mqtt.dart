import 'package:mqtt_client/mqtt_client.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:mqtt_client/mqtt_server_client.dart';
import 'package:homeflow/shared/constants.dart';
import 'package:homeflow/models/user.dart';
import 'dart:async';

class MqttService {
  Future<MqttServerClient> initMqtt(String userUid, String token) async {
    Completer<MqttServerClient> completer = Completer();
    MqttServerClient client =
        MqttServerClient.withPort(MQTT_HOST, userUid, MQTT_PORT);
    client.logging(on: true);
    client.onConnected = () {
      print("Connected to MQTT");
      completer.complete(client);
    };
    client.onDisconnected = () {
      print("Disconnected from MQTT");
    };
    client.onSubscribeFail = (topic) {
      print("Failed subscribe to $topic");
    };

    client.connectionMessage = MqttConnectMessage()
        .authenticateAs(userUid, token)
        .keepAliveFor(60)
        .startClean()
        .withWillQos(MqttQos.atLeastOnce);
    try {
      await client.connect();
    } catch (e) {
      print("Exception $e");
      client.disconnect();
    }
    client.updates.listen((List<MqttReceivedMessage<MqttMessage>> c) {
      final MqttPublishMessage message = c[0].payload;
      final payload =
          MqttPublishPayload.bytesToStringAsString(message.payload.message);

      print('Received message:$payload from topic: ${c[0].topic}>');
    });

    return completer.future;
  }

  void _onError(
      dynamic data, FirebaseUser firebaseUser, auth.User currentUser) async {
    print("Error $data}");
    await Future.delayed(Duration(milliseconds: 1000));
    print("Reconnecting!");
  }
}
