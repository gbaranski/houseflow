import 'package:flutter/material.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

class DeviceIcon extends StatelessWidget {
  final String deviceName;

  DeviceIcon(this.deviceName);

  Widget build(BuildContext context) {
    switch (this.deviceName) {
      case "ALARMCLOCK":
        {
          return Icon(MdiIcons.alarm);
        }
      case "WATERMIXER":
        {
          return Icon(MdiIcons.waterPump);
        }
      default:
        {
          return Icon(MdiIcons.alertCircle);
        }
    }
  }
}
