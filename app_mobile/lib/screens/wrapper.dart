import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/user.dart';
import 'package:houseflow/screens/auth/init_user.dart';
import 'package:houseflow/screens/auth/sign_in.dart';
import 'package:houseflow/screens/home/home.dart';
import 'package:houseflow/screens/splash_screen/splash_screen.dart';
import 'package:houseflow/services/auth.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:provider/provider.dart';
import 'package:houseflow/services/device_actions.dart';

import 'splash_screen/splash_screen.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(builder: (context, authService, child) {
      if (authService.authStatus == AuthStatus.NOT_DETERMINED)
        return SplashScreen();

      if (authService.authStatus == AuthStatus.NOT_LOGGED_IN) return SignIn();
      DeviceActions.initialize(authService.currentUser);
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
          return Home();
        },
      );
    });
  }
}
