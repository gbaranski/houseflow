import 'package:homeflow/models/user.dart';
import 'package:homeflow/screens/auth/sign_in.dart';
import 'package:homeflow/screens/home/home.dart';
import 'package:homeflow/screens/splash_screen/splash_screen.dart';
import 'package:homeflow/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/material.dart';
import 'package:homeflow/services/firebase.dart';
import 'package:homeflow/services/mqtt.dart';
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
      return FutureBuilder(
        future: FirebaseService.convertToFirebaseUser(authModel.currentUser),
        builder: (BuildContext context, AsyncSnapshot<FirebaseUser> snapshot) {
          if (snapshot.connectionState != ConnectionState.done ||
              !snapshot.hasData ||
              authModel.currentUser.getIdToken == null) {
            return SplashScreen();
          }

          authModel.firebaseUser = snapshot.data;

          print("Firebase user ${authModel.firebaseUser}");

          final MqttService mqttService = MqttService(
              getToken: authModel.currentUser.getIdToken,
              userUid: authModel.currentUser.uid);

          return FutureBuilder<MqttClient>(
            future: mqttService.connect(),
            builder:
                (BuildContext context, AsyncSnapshot<MqttClient> snapshot) {
              if (snapshot.connectionState == ConnectionState.done) {
                if (snapshot.hasData) {
                  return Home();
                }
              }
              return SplashScreen();
            },
          );
        },
      );
    });
  }
}
