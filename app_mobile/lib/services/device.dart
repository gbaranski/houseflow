import 'dart:core';
import 'package:app_mobile/models/device.dart';
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

  Future<List<FirebaseDevice>> init(
      FirebaseUser firebaseUser, auth.User currentUser) async {
    final token = await currentUser.getIdToken(true);

    _webSocketChannel = IOWebSocketChannel.connect('ws://192.168.1.10:8001',
        headers: {"sec-websocket-protocol": token});
    return await getFirebaseDevices(firebaseUser);
  }

  Stream<dynamic> get webSocketChannel {
    _webSocketChannel.sink.add("someRandomMessage");
    return _webSocketChannel.stream;
  }
}
