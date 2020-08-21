import 'package:flutter/material.dart';
import 'package:app_mobile/devices/alarmclock.dart';
import 'package:app_mobile/devices/watermixer.dart';

class _DashboardState extends State<Dashboard> {
  final List<Widget> _devices = [
    Alarmclock(),
    Divider(),
    Watermixer(),
  ];

  @override
  Widget build(BuildContext build) {
    return ListView(
      children: _devices,
    );
  }
}

class Dashboard extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _DashboardState();
  }
}
