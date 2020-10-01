import 'package:flutter/material.dart';

class WatermixerData {
  int finishMixTimestamp;

  factory WatermixerData.fromJson(Map<String, dynamic> json) {
    return WatermixerData(
      finishMixTimestamp: json["finishMixTimestamp"],
    );
  }
  Map<String, dynamic> toJson() {
    return {
      'finishMixTimestamp': finishMixTimestamp,
    };
  }

  WatermixerData({@required this.finishMixTimestamp});
}
