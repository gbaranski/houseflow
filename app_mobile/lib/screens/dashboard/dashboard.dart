import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/screens/devices/relayCard.dart';
import 'package:houseflow/screens/requests_history/requests_history.dart';
import 'package:houseflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:provider/provider.dart';

class Dashboard extends StatefulWidget {
  @override
  _DashboardState createState() => _DashboardState();
}

const tenMinutesInMillis = 1000 * 10 * 60;

class _DashboardState extends State<Dashboard> {
  Widget device(BuildContext context, String uid) {
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

  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (context, authModel, child) {
        return LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) =>
              SingleChildScrollView(
            child: ConstrainedBox(
              constraints: BoxConstraints(
                  minWidth: constraints.minWidth,
                  minHeight: constraints.minHeight),
              child: IntrinsicHeight(
                child: Container(
                  alignment: Alignment.center,
                  child: Column(children: [
                    if (authModel.firebaseUser.devices.length < 1)
                      (Container(
                          margin: const EdgeInsets.symmetric(horizontal: 20),
                          child: const Text(
                              "You don't have any devices, if you feel thats an issue, contact us"))),
                    Wrap(
                        children: authModel.firebaseUser.devices
                            .map((firebaseDevice) {
                      return device(
                        context,
                        firebaseDevice.uid,
                      );
                    }).toList()),
                    Expanded(
                      child: RequestsHistory(
                        snapshotsStreams:
                            FirebaseService.firebaseDevicesHistoryStream(
                                authModel.firebaseUser.devices),
                        firebaseUserDevices: authModel.firebaseUser.devices,
                      ),
                    ),
                  ]),
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}
