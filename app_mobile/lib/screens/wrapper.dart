import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:houseflow/models/user.dart';
import 'package:houseflow/screens/auth/init_user.dart';
import 'package:houseflow/screens/auth/sign_in.dart';
import 'package:houseflow/screens/home/home.dart';
import 'package:houseflow/screens/splash_screen/splash_screen.dart';
import 'package:houseflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'splash_screen/splash_screen.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
        stream: AuthService.user,
        builder: (context, AsyncSnapshot<auth.User> currentUserSnapshot) {
          print("New snapshot data: ${currentUserSnapshot.data}");
          if (currentUserSnapshot.connectionState != ConnectionState.active)
            return SplashScreen();

          if (currentUserSnapshot.data == null) {
            AuthService.authStatus = AuthStatus.NOT_LOGGED_IN;
            MqttService.disconnectDueToSignOut();
            return SignIn();
          } else {
            AuthService.authStatus = AuthStatus.NOT_RETREIVED_FIRESTORE;
            AuthService.currentUser = currentUserSnapshot.data;
          }
          print("AuthState: ${AuthService.authStatus}");

          return StreamBuilder(
            stream: FirebaseService.firebaseUserStream(AuthService.currentUser),
            builder: (BuildContext context,
                AsyncSnapshot<DocumentSnapshot> firebaseUserSnapshot) {
              if (!firebaseUserSnapshot.hasData) {
                return SplashScreen();
              }
              if (firebaseUserSnapshot.data.data() == null) {
                print("Received, redirecting to init user");
                return InitUser(
                  currentUser: currentUserSnapshot.data,
                );
              }
              AuthService.firebaseUser =
                  FirebaseUser.fromMap(firebaseUserSnapshot.data.data());

              print("Firebase user ${AuthService.firebaseUser}");

              FirebaseService.initFcm(context);

              if (MqttService.mqttClient != null &&
                  MqttService.mqttClient.connectionStatus.state ==
                      MqttConnectionState.connected) {
                print(
                    "Skipping creating new MQTT servie bcs its already created");
                return Home();
              }

              final MqttService mqttService = MqttService(
                  getToken: AuthService.currentUser.getIdToken,
                  userUid: AuthService.currentUser.uid);

              return new FutureBuilder<MqttClient>(
                future: mqttService.connect(),
                builder:
                    (BuildContext context, AsyncSnapshot<MqttClient> snapshot) {
                  if (snapshot.connectionState == ConnectionState.done &&
                      snapshot.hasData) {
                    return Home();
                  }
                  return SplashScreen();
                },
              );
            },
          );
        });
  }
}
