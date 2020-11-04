import 'package:flutter/material.dart';

import '../device.dart';

class LightData {
  bool currentState;

  factory LightData.fromJson(Map<String, dynamic> json) {
    return LightData(
      currentState: json["currentState"],
    );
  }

  static DeviceRequestDevice getDeviceRequest({@required String uid}) {
    return DeviceRequestDevice(
      uid: uid,
      action: "toggle",
      gpio: 1,
    );
  }

  LightData({@required this.currentState});
}
