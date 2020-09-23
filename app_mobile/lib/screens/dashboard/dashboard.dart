import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:homeflow/models/device.dart';
import 'package:homeflow/services/auth.dart';
import 'package:homeflow/services/device.dart';
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
  List<FirebaseDevice> firebaseDevices = [];

  Widget deviceWidget(BuildContext context, FirebaseDevice firebaseDevice) {
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
  }

  Widget inactiveDevice(BuildContext context, FirebaseDevice device) {
    return Text("${device.type} inactive");
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (context, deviceModel, child) {
        return StreamBuilder(
            stream: FirebaseService.getFirebaseDevicesQueries(
                    deviceModel.firebaseUser)
                .snapshots(),
            builder:
                (BuildContext context, AsyncSnapshot<QuerySnapshot> snapshot) {
              print("Snapshot ${snapshot.data}");
              if (snapshot.hasError) {
                return Text("Something went wrong");
              }
              if (snapshot.connectionState == ConnectionState.waiting) {
                return CircularProgressIndicator();
              }

              return Container(
                child: Column(children: [
                  Expanded(
                    child: ListView.builder(
                        itemCount: snapshot.data.docs.length,
                        itemBuilder: (context, index) {
                          final data = snapshot.data.docs[index].data();
                          final device = FirebaseDevice(
                              data: data['data'],
                              ip: data['ip'],
                              status: data['status'],
                              type: data['type'],
                              uid: data['uid']);
                          if (device.status) {
                            return deviceWidget(context, device);
                          } else {
                            return InactiveDevice(device);
                          }
                        }),
                  ),
                ]),
              );
            });
      },
    );
  }
}
