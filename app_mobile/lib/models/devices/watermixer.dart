import 'package:flutter/material.dart';

import '../device.dart';

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

  static DeviceTopic getStartMixingTopic(String uid) {
    return new DeviceTopic(
        request: '$uid/event/relay1/request',
        response: '$uid/event/relay1/response');
  }

  WatermixerData({@required this.finishMixTimestamp});
}
