import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:houseflow/widgets/devices/relayCard.dart';

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
        // if (!firebaseDevice.status) return InactiveDevice(firebaseDevice);

        final iconData = getDeviceIcon(firebaseDevice.type);
        switch (firebaseDevice.type) {
          case 'WATERMIXER':
            return RelayCard(
              cardColor: Color.fromRGBO(79, 119, 149, 1),
              firebaseDevice: firebaseDevice,
              iconData: iconData,
              getNewDeviceData: () =>
                  DateTime.now().millisecondsSinceEpoch + tenMinutesInMillis,
            );
          case 'GATE':
            return RelayCard(
              cardColor: Color.fromRGBO(103, 151, 109, 1),
              firebaseDevice: firebaseDevice,
              iconData: iconData,
              getNewDeviceData: () => DateTime.now().millisecondsSinceEpoch,
            );
          case 'GARAGE':
            return RelayCard(
              cardColor: Color.fromRGBO(183, 111, 110, 1),
              firebaseDevice: firebaseDevice,
              iconData: iconData,
              getNewDeviceData: () => DateTime.now().millisecondsSinceEpoch,
            );

          default:
            {
              return const Text("Some error occured");
            }
        }
      },
    );
  }
}
