import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:homeflow/models/device.dart';
import 'package:homeflow/screens/devices/inactive.dart';
import 'package:homeflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:homeflow/services/firebase.dart';
import 'package:homeflow/shared/constants.dart';
import 'package:homeflow/utils/misc.dart';
import 'package:provider/provider.dart';
import 'package:homeflow/screens/devices/watermixer.dart';

class Dashboard extends StatefulWidget {
  @override
  _DashboardState createState() => _DashboardState();
}

class _DashboardState extends State<Dashboard> {
  Widget device(BuildContext context, String uid) {
    return StreamBuilder(
      stream: FirebaseService.getFirebaseDeviceSnapshot(uid),
      builder:
          (BuildContext context, AsyncSnapshot<DocumentSnapshot> snapshot) {
        if (snapshot.hasError) return Text("Error occured");
        if (snapshot.connectionState == ConnectionState.waiting)
          return CircularProgressIndicator();
        final Map<String, dynamic> data = snapshot.data.data();
        final FirebaseDevice firebaseDevice = FirebaseDevice.fromMap(data);
        if (!firebaseDevice.status) return InactiveDevice(firebaseDevice);

        switch (firebaseDevice.type) {
          case 'WATERMIXER':
            {
              return Watermixer(
                firebaseDevice: firebaseDevice,
              );
            }
            break;
          default:
            {
              return Text("Some error occured");
            }
        }
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (context, authModel, child) {
        return Container(
          alignment: Alignment.center,
          child: Column(children: [
            if (authModel.firebaseUser.devices.length < 1)
              (Container(
                  margin: const EdgeInsets.symmetric(horizontal: 20),
                  child: Text(
                      "You don't have any devices, if you feel thats an issue, contact us"))),
            Expanded(
              child: ListView.builder(
                  itemCount: authModel.firebaseUser.devices.length,
                  itemBuilder: (context, index) {
                    return Container(
                      margin: const EdgeInsets.symmetric(
                          horizontal: 8, vertical: 15),
                      child: device(
                          context, authModel.firebaseUser.devices[index].uid),
                    );
                  }),
            )
          ]),
        );
      },
    );
  }
}
