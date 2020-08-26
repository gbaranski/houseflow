import 'package:flutter/material.dart';

import 'index.dart';

class AlarmclockData {
  DateTime alarmTime;
  bool alarmState;
  SensorData sensor;

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
}
