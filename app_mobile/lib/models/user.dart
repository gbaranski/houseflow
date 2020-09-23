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

  // This doesn't exist in firestore, but needed to handle it later
  final bool isAnonymous;

  FirebaseUser(
      {@required this.devices,
      @required this.role,
      @required this.uid,
      @required this.isAnonymous});
}

enum AuthStatus {
  NOT_DETERMINED,
  NOT_LOGGED_IN,
  LOGGED_IN,
}
