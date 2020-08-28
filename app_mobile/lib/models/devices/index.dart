import 'package:flutter/cupertino.dart';

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

  String toReadableString() {
    String hour = this.hour.toString().padLeft(2, "0");
    String minute = this.minute.toString().padLeft(2, "0");
    return "$hour:$minute";
  }
}
