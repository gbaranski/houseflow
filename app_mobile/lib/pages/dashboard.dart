import 'package:flutter/material.dart';
import 'package:app_mobile/devices/alarmclock.dart';
import 'package:app_mobile/devices/watermixer.dart';

class _DashboardState extends State<Dashboard> {
  final List<Widget> _devices = [
    Container(
      margin: const EdgeInsets.only(left: 10.0, right: 10.0, top: 20),
      child: Column(children: [
        Alarmclock(),
      ]),
    )
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
