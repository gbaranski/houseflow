import 'package:app_mobile/models/device.dart';
import 'package:app_mobile/services/device.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:app_mobile/screens/devices/alarmclock.dart';
import 'package:app_mobile/screens/devices/watermixer.dart';

class Dashboard extends StatelessWidget {
  Widget deviceWidget(BuildContext context, ActiveDevice activeDevice) {
    switch (activeDevice.type) {
      case 'ALARMCLOCK':
        {
          return Alarmclock(
            activeDevice: activeDevice,
          );
        }
        break;
      case 'WATERMIXER':
        {
          return Watermixer(activeDevice: activeDevice);
        }
        break;
      default:
        {
          return Text("Some error occured");
        }
    }
  }

  Widget inactiveDevice(BuildContext context, FirebaseDevice device) {
    return Text("${device.type} inactive");
  }

  Widget deviceList(BuildContext context) {
    return Consumer<DeviceService>(
      builder: (context, deviceModel, child) {
        return Column(children: [
          Expanded(
            child: ListView.builder(
                itemCount: deviceModel.firebaseDevices.length,
                itemBuilder: (context, index) {
                  final firebaseDevice = deviceModel.firebaseDevices[index];
                  final active = deviceModel.activeDevices
                      .any((element) => element.uid == firebaseDevice.uid);
                  if (active) {
                    final activeDevice = deviceModel.activeDevices.singleWhere(
                        (_device) => _device.uid == firebaseDevice.uid);
                    return deviceWidget(context, activeDevice);
                  } else {
                    return inactiveDevice(context, firebaseDevice);
                  }
                }),
          ),
        ]);
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: deviceList(context),
    );
  }
}
