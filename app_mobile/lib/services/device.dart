import 'package:app_mobile/models/device.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:app_mobile/models/user.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

class DeviceService extends ChangeNotifier {
  WebSocketChannel webSocketChannel;
  List activeDevices;
  List firebaseDevices;

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
}
