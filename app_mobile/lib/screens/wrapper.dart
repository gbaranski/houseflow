import 'package:app_mobile/models/user.dart';
import 'package:app_mobile/screens/auth/sign_in.dart';
import 'package:app_mobile/screens/home/home.dart';
import 'package:app_mobile/screens/splash_screen/splash_screen.dart';
import 'package:app_mobile/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final user = Provider.of<User>(context);

    return Consumer<AuthService>(
      builder: (context, model, child) {
        print(model.authStatus);
        if (model.authStatus == AuthStatus.NOT_DETERMINED) {
          return SplashScreen();
        }
        if (user == null) {
          return SignIn();
        } else {
          print(user);
          return Home();
        }
      },
    );
  }
}
