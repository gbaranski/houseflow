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
  Map<String, dynamic> toJson() {
    return {
      'data': data,
      'ip': ip,
      'status': status,
      'type': type,
      'uid': uid,
    };
  }

  FirebaseDevice(
      {@required this.data,
      @required this.ip,
      @required this.status,
      @required this.type,
      @required this.uid});
}

class RequestTopic {
  final String request;
  final String response;

  RequestTopic({@required this.request, @required this.response});
}
