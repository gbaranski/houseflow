import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:homeflow/models/device.dart';
import 'package:homeflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:homeflow/services/firebase.dart';
import 'package:provider/provider.dart';
import 'package:homeflow/screens/devices/watermixer.dart';
import 'package:homeflow/screens/devices/inactive.dart';

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

  Widget inactiveDevice(BuildContext context, FirebaseDevice device) {
    return Text("${device.type} inactive");
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (context, authModel, child) {
        return Container(
          child: Column(children: [
            Expanded(
              child: ListView.builder(
                  itemCount: authModel.firebaseUser.devices.length,
                  itemBuilder: (context, index) {
                    return device(
                        context, authModel.firebaseUser.devices[index].uid);
                  }),
            ),
          ]),
        );
      },
    );
  }
}
