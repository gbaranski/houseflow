import 'package:app_mobile/models/device.dart';
import 'package:flutter/material.dart';

class Alarmclock extends StatelessWidget {
  final ActiveDevice activeDevice;

  Alarmclock({@required this.activeDevice});

  @override
  Widget build(BuildContext context) {
    return Text("Alarmclock UID ${activeDevice.uid}");
  }
}
