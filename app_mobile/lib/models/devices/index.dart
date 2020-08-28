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
