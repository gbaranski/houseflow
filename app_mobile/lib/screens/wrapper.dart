import 'package:app_mobile/screens/auth/auth_screen.dart';
import 'package:app_mobile/screens/home/home.dart';
import 'package:firebase_auth/firebase_auth.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Wrapper extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final user = Provider.of<User>(context);

    if (user == null) {
      return AuthScreen();
    } else {
      print(user);
      return Home();
    }
  }
}
