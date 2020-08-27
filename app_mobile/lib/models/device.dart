import 'package:flutter/material.dart';

class FirebaseDevice {
  String uid;
  String type;

  FirebaseDevice({@required this.uid, @required this.type});
}

class ActiveDevice extends FirebaseDevice {
  String ip;
  dynamic data;

  ActiveDevice(
      {@required this.ip, @required this.data, @required uid, @required type})
      : super(uid: uid, type: type);
}
