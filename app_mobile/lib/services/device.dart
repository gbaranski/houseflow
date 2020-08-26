import 'dart:convert';
import 'dart:core';
import 'package:app_mobile/models/device.dart';
import 'package:app_mobile/models/misc.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/material.dart';
import 'package:app_mobile/models/user.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class DeviceService extends ChangeNotifier {
  WebSocketChannel _webSocketChannel;
  List<ActiveDevice> _activeDevices = [];
  List<FirebaseDevice> firebaseDevices = [];

  List<FirebaseDevice> get activeDevices {
    return _activeDevices;
  }

  Future<List<FirebaseDevice>> getFirebaseDevices(
      FirebaseUser firebaseUser) async {
    final List<Future<DocumentSnapshot>> snapshots = firebaseUser.devices
        .map((_device) => (_device as DocumentReference).get())
        .toList();

    return (await Future.wait(snapshots)).map((snapshot) {
      final data = snapshot.data();
      return FirebaseDevice(uid: data['uid'], type: data['type']);
    }).toList();
  }

  WebSocketChannel _initWebsocket(String token) {
    return IOWebSocketChannel.connect('ws://192.168.1.10:8001',
        headers: {"sec-websocket-protocol": token});
  }

  void _onData(dynamic data) {
    try {
      Map responseMap = jsonDecode(data);
      var response = ServerResponse.fromJson(responseMap);

      switch (response.requestType) {
        case 'DATA':
          {
            print(response.data);
            final parsedActiveDeviceList = response.data.cast<ActiveDevice>();
            print(parsedActiveDeviceList);
            print("Received response for data");
          }
          break;
        default:
          {
            print("Unhandled response type");
          }
          break;
      }
    } catch (e) {
      print(e.toString());
    }
  }

  void _onError(dynamic data) {
    print("Error $data}");
  }

  Future<List<FirebaseDevice>> init(
      FirebaseUser firebaseUser, auth.User currentUser) async {
    final token = await currentUser.getIdToken(true);
    _webSocketChannel = _initWebsocket(token);
    _webSocketChannel.stream.listen(_onData, onError: _onError);
    return await getFirebaseDevices(firebaseUser);
  }
}
