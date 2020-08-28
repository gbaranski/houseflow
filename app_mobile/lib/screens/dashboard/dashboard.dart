import 'package:control_home/models/device.dart';
import 'package:control_home/services/device.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:control_home/screens/devices/alarmclock.dart';
import 'package:control_home/screens/devices/watermixer.dart';
import 'package:control_home/screens/devices/inactive.dart';

class Dashboard extends StatelessWidget {
  Widget deviceWidget(BuildContext context, String uid, String type) {
    switch (type) {
      case 'ALARMCLOCK':
        {
          return Alarmclock(
            uid: uid,
          );
        }
        break;
      case 'WATERMIXER':
        {
          return Watermixer(uid: uid);
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
        return Container(
          child: Column(children: [
            Expanded(
              child: ListView.builder(
                  itemCount: deviceModel.firebaseDevices.length,
                  itemBuilder: (context, index) {
                    final firebaseDevice = deviceModel.firebaseDevices[index];
                    final active = deviceModel.activeDevices
                        .any((element) => element.uid == firebaseDevice.uid);
                    if (active) {
                      final activeDevice = deviceModel.activeDevices
                          .singleWhere(
                              (_device) => _device.uid == firebaseDevice.uid);
                      return deviceWidget(
                          context, activeDevice.uid, activeDevice.type);
                    } else {
                      return InactiveDevice(firebaseDevice);
                    }
                  }),
            ),
          ]),
        );
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
