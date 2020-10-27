import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
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
    return StreamBuilder(
        stream: AuthService.user,
        builder: (context, AsyncSnapshot<auth.User> currentUserSnapshot) {
          print("New snapshot data: ${currentUserSnapshot.data}");

          final MqttService mqttService =
              Provider.of<MqttService>(context, listen: false);
          if (currentUserSnapshot.connectionState != ConnectionState.active)
            return SplashScreen();

          if (currentUserSnapshot.data == null) {
            AuthService.authStatus = AuthStatus.NOT_LOGGED_IN;
            mqttService.disconnect("sign out");
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
              final connect = () => mqttService.connect(
                  userUid: AuthService.currentUser.uid,
                  token: AuthService.currentUser.getIdToken());

              return StreamBuilder<ConnectionStatus>(
                stream: mqttService.streamController.stream,
                initialData: ConnectionStatus.not_attempted,
                builder: (BuildContext context, stream) {
                  switch (stream.data) {
                    case ConnectionStatus.connected:
                      return Home();
                    case ConnectionStatus.disconnected:
                      connect();
                      return ErrorScreen(
                        reason: "Disconnected",
                      );
                    case ConnectionStatus.failed:
                      connect();
                      return ErrorScreen(
                        reason: "Could not connect",
                      );
                    case ConnectionStatus.attempts_exceeded:
                      return ErrorScreen(
                        reason: "Attempts exceeded",
                      );
                    case ConnectionStatus.not_attempted:
                      connect();
                      return SplashScreen();
                  }

                  return Home();
                },
                // builder: (BuildContext context,
                //     AsyncSnapshot<MqttClientConnectionStatus> snapshot) {
                //   if (snapshot.hasData) {
                //     if (snapshot.data.state == MqttConnectionState.connected) {
                //       return Home();
                //     } else {
                //       return ErrorScreen(
                //           reason: "Could not connect to our servers");
                //     }
                //   }
                //   return SplashScreen();
                // },
              );
            },
          );
        });
  }
}
