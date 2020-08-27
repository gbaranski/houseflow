import 'package:app_mobile/models/device.dart';
import 'package:flutter/material.dart';

class Watermixer extends StatelessWidget {
  final ActiveDevice activeDevice;

  Watermixer({@required this.activeDevice});

  @override
  Widget build(BuildContext context) {
    return Text("Watermixer UID ${activeDevice.uid}");
  }
}
