import 'package:homeflow/models/user.dart';
import 'package:homeflow/screens/auth/sign_in.dart';
import 'package:homeflow/screens/home/home.dart';
import 'package:homeflow/screens/splash_screen/splash_screen.dart';
import 'package:homeflow/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/material.dart';
import 'package:homeflow/services/mqtt.dart';
import 'package:provider/provider.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final user = Provider.of<auth.User>(context);

    return Consumer<AuthService>(builder: (context, authModel, child) {
      print("AuthState: ${authModel.authStatus}");
      if (authModel.authStatus == AuthStatus.NOT_DETERMINED) {
        return SplashScreen();
      }
      if (user == null) {
        return SignIn();
      } else {
        print("CurrentUser: $user");

        return FutureProvider<MqttService>(
          create: (_) async => MqttService(
              token: await authModel.currentUser.getIdToken(),
              userUid: authModel.currentUser.uid),
          child: Home(),
        );
      }
    });
  }
}
