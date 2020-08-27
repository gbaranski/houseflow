import 'package:flutter/material.dart';

class WatermixerData {
  int remainingSeconds;
  bool isTimerOn;

  factory WatermixerData.fromJson(Map<String, dynamic> json) {
    return WatermixerData(
      remainingSeconds: json["remainingSeconds"],
      isTimerOn: json["isTimerOn"],
    );
  }
  WatermixerData({@required this.remainingSeconds, @required this.isTimerOn});
}
