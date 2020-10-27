import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/user.dart';
import 'package:houseflow/screens/auth/init_user.dart';
import 'package:houseflow/screens/auth/sign_in.dart';
import 'package:houseflow/screens/error_screen/error_screen.dart';
import 'package:houseflow/screens/home/home.dart';
import 'package:houseflow/screens/splash_screen/splash_screen.dart';
import 'package:houseflow/services/auth.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:provider/provider.dart';

import 'splash_screen/splash_screen.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(builder: (context, authService, child) {
      final MqttService mqttService =
          Provider.of<MqttService>(context, listen: false);

      if (authService.authStatus == AuthStatus.NOT_DETERMINED) {
        mqttService.resetConnectionStatus();
        return SplashScreen();
      }

      if (authService.authStatus == AuthStatus.NOT_LOGGED_IN) {
        mqttService.resetConnectionStatus();
        mqttService.disconnect('not logged in');
        return SignIn();
      }
      print(authService.authStatus);
      print(authService.currentUser);

      return StreamBuilder<DocumentSnapshot>(
        stream: FirebaseService.firebaseUserStream(authService.currentUser),
        builder: (context, snapshot) {
          if (!snapshot.hasData) return SplashScreen();
          if (!snapshot.data.exists) {
            return InitUser(
              currentUser: authService.currentUser,
            );
          }

          authService.firebaseUser = FirebaseUser.fromMap(snapshot.data.data());

          return Consumer<MqttService>(
            builder: (context, mqttService, child) {
              final connect = () => mqttService.connect(
                  userUid: authService.currentUser.uid,
                  token: authService.currentUser.getIdToken());

              switch (mqttService.connectionStatus) {
                case ConnectionStatus.connected:
                  return Home();
                case ConnectionStatus.disconnected:
                  connect();
                  return SplashScreen();
                  break;
                case ConnectionStatus.failed:
                  connect();
                  return ErrorScreen(
                    reason: "Could not connect",
                  );
                  break;
                case ConnectionStatus.attempts_exceeded:
                  return ErrorScreen(
                    reason: "Attempts exceeded",
                  );
                  break;
                case ConnectionStatus.not_attempted:
                  connect();
                  return SplashScreen();
                  break;
                default:
                  return ErrorScreen(
                    reason: "ConnectionStatus is in unexpected state",
                  );
              }
            },
          );
        },
      );
    });
  }
}
