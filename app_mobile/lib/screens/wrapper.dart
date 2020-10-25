import 'package:cloud_firestore/cloud_firestore.dart';
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
import 'package:provider/provider.dart';

import 'splash_screen/splash_screen.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(builder: (context, authModel, child) {
      print("AuthState: ${authModel.authStatus}");

      if (authModel.authStatus == AuthStatus.NOT_DETERMINED) {
        return SplashScreen();
      }
      if (authModel.authStatus == AuthStatus.NOT_LOGGED_IN ||
          authModel.currentUser == null) {
        MqttService.disconnectDueToSignOut();
        return SignIn();
      }

      print("CurrentUser: ${authModel.currentUser}");

      return StreamBuilder(
        stream: FirebaseService.firebaseUserStream(authModel.currentUser),
        builder:
            (BuildContext context, AsyncSnapshot<DocumentSnapshot> snapshot) {
          if (!snapshot.hasData) {
            return SplashScreen();
          }
          if (snapshot.data.data() == null) {
            print("Received, redirecting to init user");
            return InitUser();
          }
          authModel.firebaseUser = FirebaseUser.fromMap(snapshot.data.data());

          print("Firebase user ${authModel.firebaseUser}");

          FirebaseService.initFcm(context);

          if (MqttService.mqttClient != null &&
              MqttService.mqttClient.connectionStatus.state ==
                  MqttConnectionState.connected) {
            print("Skipping creating new MQTT servie bcs its already created");
            return Home();
          }

          final MqttService mqttService = MqttService(
              getToken: authModel.currentUser.getIdToken,
              userUid: authModel.currentUser.uid);

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
