import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/light.dart';
import 'package:houseflow/models/devices/relay.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:houseflow/widgets/devices/device_card.dart';
import 'package:houseflow/widgets/devices/device_card_wrapper.dart';

const tenMinutesInMillis = 1000 * 10 * 60;

class Device extends StatelessWidget {
  const Device({
    Key key,
    @required this.context,
    @required this.uid,
  }) : super(key: key);

  final BuildContext context;
  final String uid;

  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
      stream: FirebaseService.getFirebaseDeviceSnapshot(uid),
      builder:
          (BuildContext context, AsyncSnapshot<DocumentSnapshot> snapshot) {
        if (snapshot.hasError) return Text("Error occured");
        if (snapshot.connectionState == ConnectionState.waiting)
          return Container();
        final Map<String, dynamic> data = snapshot.data.data();
        final FirebaseDevice firebaseDevice = FirebaseDevice.fromMap(data);

        switch (firebaseDevice.type) {
          case 'WATERMIXER':
            return DeviceCardWrapper(
              color: Color.fromRGBO(79, 119, 149, 1),
              firebaseDevice: firebaseDevice,
              deviceRequestDevice:
                  RelayData.getDeviceRequest(uid: firebaseDevice.uid),
            );
          case 'GATE':
            return DeviceCardWrapper(
              color: Color.fromRGBO(103, 151, 109, 1),
              firebaseDevice: firebaseDevice,
              deviceRequestDevice:
                  RelayData.getDeviceRequest(uid: firebaseDevice.uid),
            );
          case 'GARAGE':
            return DeviceCardWrapper(
              color: Color.fromRGBO(183, 111, 110, 1),
              firebaseDevice: firebaseDevice,
              deviceRequestDevice:
                  RelayData.getDeviceRequest(uid: firebaseDevice.uid),
            );
          case 'LIGHT':
            return DeviceCardWrapper(
              color: Color(0xFFffa000),
              firebaseDevice: firebaseDevice,
              deviceRequestDevice:
                  LightData.getDeviceRequest(uid: firebaseDevice.uid),
            );
          default:
            {
              return DeviceCard(
                onValidTap: () {
                  const snackbar = SnackBar(
                      content: Text(
                          "Could not perform action, unrecognized device, please update app"));
                  Scaffold.of(context).showSnackBar(snackbar);
                },
                iconData: Icons.error,
                color: Colors.red.shade300,
                firebaseDevice: firebaseDevice,
              );
            }
        }
      },
    );
  }
}
