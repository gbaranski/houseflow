import 'package:flutter/material.dart';

class FirebaseUserDevice {
  final bool notification;
  final String uid;

  FirebaseUserDevice({@required this.notification, @required this.uid});
}

class FirebaseUser {
  final List<FirebaseUserDevice> devices;
  final String role;
  final String uid;
  final String username;

  // This doesn't exist in firestore, but needed to handle it later
  final bool isAnonymous;

  factory FirebaseUser.fromMap(Map<String, dynamic> map) {
    final userDevices = (map['devices'] as List<dynamic>)
        .map((device) => FirebaseUserDevice(
            notification: device['notification'], uid: device['uid']))
        .toList();

    return FirebaseUser(
        uid: map['uid'],
        role: map['role'],
        devices: userDevices,
        isAnonymous: false,
        username: map['username']);
  }

  FirebaseUser({
    @required this.devices,
    @required this.role,
    @required this.uid,
    @required this.isAnonymous,
    @required this.username,
  });
}

enum AuthStatus {
  NOT_DETERMINED,
  NOT_RETREIVED_FIRESTORE,
  NOT_LOGGED_IN,
  LOGGED_IN,
}
