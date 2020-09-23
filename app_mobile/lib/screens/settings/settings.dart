import 'package:homeflow/models/device.dart';
import 'package:homeflow/services/auth.dart';
import 'package:homeflow/shared/device_icon.dart';
import 'package:homeflow/utils/misc.dart';
import 'package:homeflow/services/device.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Settings extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(builder: (context, authModel, child) {
      final deviceModel = Provider.of<DeviceService>(context, listen: false);
      deviceModel.getFirebaseDevices(authModel.firebaseUser);
      return ListView.builder(
          itemCount: deviceModel.firebaseDevices.length,
          itemBuilder: (context, index) {
            final FirebaseDevice firebaseDevice =
                deviceModel.firebaseDevices[index];

            return ExpansionTile(
              leading: DeviceIcon(firebaseDevice.type),
              title: Text(upperFirstCharacter(firebaseDevice.type)),
              children: [
                Text("UID: ${firebaseDevice.uid}"),
              ],
            );
          });
    });
  }
}
