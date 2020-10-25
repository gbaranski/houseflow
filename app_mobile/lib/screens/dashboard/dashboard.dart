import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/screens/devices/relayCard.dart';
import 'package:houseflow/screens/requests_history/requests_history.dart';
import 'package:houseflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/utils/misc.dart';

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

  final List<DeviceHistory> _deviceHistory = [];

  Future<void> updateDeviceHistory() async {
    final snapshot = await FirebaseService.getFirebaseDeviceHistory(
        AuthService.firebaseUser.devices);
    final deviceHistory = snapshot.docs
        .map((doc) => DeviceHistory.fromJson(doc.data(), doc.id))
        .toList();
    setState(() {
      _deviceHistory.addAll(deviceHistory);
    });
  }

  @override
  void initState() {
    super.initState();
    updateDeviceHistory();
  }

  @override
  Widget build(BuildContext context) {
    return CustomScrollView(
        physics:
            AlwaysScrollableScrollPhysics().applyTo(BouncingScrollPhysics()),
        slivers: [
          SliverAppBar(
            title: Text("Title"),
          ),
          SliverGrid(
            gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
                childAspectRatio: 1.2, maxCrossAxisExtent: 250),
            delegate: SliverChildListDelegate(AuthService.firebaseUser.devices
                .map((firebaseDevice) => device(context, firebaseDevice.uid))
                .toList()),
          ),
          SliverList(
            delegate: SliverChildListDelegate(_deviceHistory
                .map((deviceRequest) => SingleDeviceHistory(
                      deviceRequest: deviceRequest,
                    ))
                .toList()),
          )
        ]);
  }
}
