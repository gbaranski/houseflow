import 'package:app_mobile/models/device.dart';
import 'package:app_mobile/models/user.dart';
import 'package:app_mobile/screens/auth/sign_in.dart';
import 'package:app_mobile/screens/home/home.dart';
import 'package:app_mobile/screens/splash_screen/splash_screen.dart';
import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/services/device.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Wrapper extends StatelessWidget {
  Widget buildTargetScreen(
      BuildContext context, FirebaseUser firebaseUser, auth.User currentUser) {
    final deviceService = Provider.of<DeviceService>(context, listen: false);

    final init = deviceService.init(firebaseUser, currentUser);
    return FutureBuilder<List<FirebaseDevice>>(
        future: init,
        builder: (BuildContext context,
            AsyncSnapshot<List<FirebaseDevice>> snapshot) {
          if (snapshot.hasData) {
            print("Snapshot data received");
            deviceService.firebaseDevices = snapshot.data;
            print(deviceService.firebaseDevices);
            return Home();
          }
          return SplashScreen();
        });
  }

  @override
  Widget build(BuildContext context) {
    final user = Provider.of<auth.User>(context);

    return Consumer<AuthService>(builder: (context, authModel, child) {
      print(authModel.authStatus);
      if (authModel.authStatus == AuthStatus.NOT_DETERMINED) {
        return SplashScreen();
      }
      if (user == null) {
        return SignIn();
      } else {
        print(user);
        return ChangeNotifierProvider(
            create: (_) => DeviceService(),
            builder: (context, child) {
              print("ChangeNotifierProvider");
              return buildTargetScreen(
                  context, authModel.firebaseUser, authModel.currentUser);
            });
      }
    });
  }
}
