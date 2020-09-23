import 'dart:convert';
import 'dart:core';
import 'package:homeflow/models/device.dart';
import 'package:homeflow/models/misc.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:homeflow/shared/constants.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/material.dart';
import 'package:homeflow/models/user.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/status.dart';
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
    return IOWebSocketChannel.connect(WS_URL,
        headers: {"sec-websocket-protocol": token});
  }

  String sendRequest(Map<String, dynamic> request) {
    final json = jsonEncode(request);
    _webSocketChannel.sink.add(json);
    return json;
  }

  void _onData(dynamic data) {
    try {
      Map responseMap = jsonDecode(data);
      final response = ServerResponse.fromJson(responseMap);

      switch (response.requestType) {
        case 'DATA':
          {
            print("Received data");
            _activeDevices = response.data;
            notifyListeners();
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

  void _onError(
      dynamic data, FirebaseUser firebaseUser, auth.User currentUser) async {
    print("Error $data}");
    await Future.delayed(Duration(milliseconds: 1000));
    print("Reconnecting!");
    init(firebaseUser, currentUser);
  }

  Future<List<FirebaseDevice>> init(
      FirebaseUser firebaseUser, auth.User currentUser) async {
    final token = await currentUser.getIdToken(true);
    _webSocketChannel = _initWebsocket(token);
    _webSocketChannel.stream.listen(_onData, onError: (dynamic data) {
      _onError(data, firebaseUser, currentUser);
    });
    return await getFirebaseDevices(firebaseUser);
  }

  @override
  void dispose() {
    _webSocketChannel.sink.close(goingAway);
    super.dispose();
  }
}
