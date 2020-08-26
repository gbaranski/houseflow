import 'package:app_mobile/services/device.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Dashboard extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<DeviceService>(
      builder: (context, deviceModel, child) {
        return Container(
            child: Column(children: [
          StreamBuilder<dynamic>(
              stream: deviceModel.webSocketChannel,
              builder: (context, snapshot) {
                return Text(snapshot.hasData ? snapshot.data : 'No data');
              }),
          Expanded(
            child: ListView.builder(
                itemCount: deviceModel.firebaseDevices.length,
                itemBuilder: (context, index) {
                  final device = deviceModel.firebaseDevices[index];
                  final exists = deviceModel.activeDevices
                      .any((element) => element.uid == device.uid);
                  return Text("${device.type} Active: $exists");
                }),
          ),
        ]));
      },
    );
  }
}
