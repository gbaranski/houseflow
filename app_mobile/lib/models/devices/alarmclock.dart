import 'package:flutter/material.dart';

import 'index.dart';

class AlarmclockData {
  DeviceDateTime alarmTime;
  bool alarmState;
  SensorData sensor;

  factory AlarmclockData.fromJson(Map<String, dynamic> json) {
    return AlarmclockData(
        alarmTime: DeviceDateTime.fromJson(json['alarmTime']),
        alarmState: json["alarmState"],
        sensor: SensorData.fromJson(json['sensor']));
  }

  AlarmclockData(
      {@required alarmState, @required this.alarmTime, @required this.sensor});
}

class SensorData {
  num temperature;
  num humidity;
  num heatIndex;

  SensorData(
      {@required this.temperature,
      @required this.humidity,
      @required this.heatIndex});

  factory SensorData.fromJson(Map<String, dynamic> json) {
    return SensorData(
      temperature: json["temperature"],
      humidity: json["humidity"],
      heatIndex: json["heatIndex"],
    );
  }
}
