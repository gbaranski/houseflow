import 'package:flutter/material.dart';

class FirebaseDevice {
  Map<String, dynamic> data;
  String ip;
  bool status;
  String type;
  String uid;

  factory FirebaseDevice.fromMap(Map<String, dynamic> map) {
    return FirebaseDevice(
        data: map['data'],
        ip: map['ip'],
        status: map['status'],
        type: map['type'],
        uid: map['uid']);
  }

  FirebaseDevice(
      {@required this.data,
      @required this.ip,
      @required this.status,
      @required this.type,
      @required this.uid});
}
