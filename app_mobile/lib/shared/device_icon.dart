import 'package:flutter/material.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

class DeviceIcon extends StatelessWidget {
  final String deviceName;

  const DeviceIcon(this.deviceName);

  Widget build(BuildContext context) {
    switch (this.deviceName) {
      case "ALARMCLOCK":
        {
          return const Icon(MdiIcons.alarm);
        }
      case "WATERMIXER":
        {
          return const Icon(MdiIcons.waterPump);
        }
      default:
        {
          return const Icon(MdiIcons.alertCircle);
        }
    }
  }
}
