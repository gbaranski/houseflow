import 'package:flutter/material.dart';

import '../device.dart';

class GateData {
  int lastOpenTimestamp;

  factory GateData.fromJson(Map<String, dynamic> json) {
    return GateData(
      lastOpenTimestamp: json["lastOpenTimestamp"],
    );
  }
  Map<String, dynamic> toJson() {
    return {
      'lastOpenTimestamp': lastOpenTimestamp,
    };
  }

  static DeviceTopic getOpenGateTopic(String uid) {
    return new DeviceTopic(
        request: '$uid/event/relay1/request',
        response: '$uid/event/relay1/response');
  }

  GateData({@required this.lastOpenTimestamp});
}
