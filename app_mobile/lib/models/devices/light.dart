import 'package:flutter/material.dart';

import '../device.dart';

class LightData {
  static DeviceRequestDevice getDeviceRequest({@required String uid}) {
    return DeviceRequestDevice(
      uid: uid,
      action: 1,
    );
  }
}
