import 'package:flutter/material.dart';

import '../device.dart';

class RelayData {
  static DeviceRequestDevice getDeviceRequest({@required String uid}) {
    return DeviceRequestDevice(
      uid: uid,
      action: "trigger",
      gpio: 1,
    );
  }
}
