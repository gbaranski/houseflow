import 'package:flutter/material.dart';

import '../device.dart';

class RelayData {
  int lastSignalTimestamp;

  factory RelayData.fromJson(Map<String, dynamic> json) {
    return RelayData(
      lastSignalTimestamp: json["lastSignalTimestamp"],
    );
  }
  Map<String, dynamic> toJson() {
    return {
      'lastSignalTimestamp': lastSignalTimestamp,
    };
  }

  static DeviceTopic getSendSignalTopic(String uid) {
    return new DeviceTopic(
        request: '$uid/event/relay1/request',
        response: '$uid/event/relay1/response');
  }

  RelayData({@required this.lastSignalTimestamp});
}
