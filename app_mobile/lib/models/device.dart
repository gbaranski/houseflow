import 'package:flutter/material.dart';

class FirebaseDevice {
  Map<String, dynamic> data;
  String ip;
  bool status;
  String type;
  String uid;

  FirebaseDevice(
      {@required this.data,
      @required this.ip,
      @required this.status,
      @required this.type,
      @required this.uid});
}
