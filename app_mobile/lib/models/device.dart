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

class DeviceDateTime {
  int hour;
  int minute;
  int second;

  DeviceDateTime(
      {@required this.hour, @required this.minute, @required this.second});

  factory DeviceDateTime.fromJson(Map<String, dynamic> json) {
    return DeviceDateTime(
      hour: json['hour'],
      minute: json['minute'],
      second: json['second'],
    );
  }

  DateTime getDateTimeObject() {
    final now = new DateTime.now();
    final isToday = now.hour < hour;
    return new DateTime(
        now.year, now.month, isToday ? now.day : now.day + 1, hour, minute);
  }

  String toReadableString() {
    String hour = this.hour.toString().padLeft(2, "0");
    String minute = this.minute.toString().padLeft(2, "0");
    return "$hour:$minute";
  }

  String timeDiff() {
    final now = new DateTime.now();
    final timeDiff = getDateTimeObject().difference(now);
    return "${timeDiff.inHours}h ${timeDiff.inMinutes % 60}m";
  }
}

class DeviceTopic {
  String request;
  String response;

  DeviceTopic({@required this.request, @required this.response});
}
