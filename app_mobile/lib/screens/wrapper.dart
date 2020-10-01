import 'package:homeflow/models/user.dart';
import 'package:homeflow/screens/auth/sign_in.dart';
import 'package:homeflow/screens/home/home.dart';
import 'package:homeflow/screens/splash_screen/splash_screen.dart';
import 'package:homeflow/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/material.dart';
import 'package:homeflow/services/mqtt.dart';
import 'package:mqtt_client/mqtt_client.dart';
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

        final MqttService mqttService = new MqttService(
            getToken: authModel.currentUser.getIdToken,
            userUid: authModel.currentUser.uid);

        return ChangeNotifierProvider<MqttService>.value(
          value: mqttService,
          child: mqttService.mqttClient != null &&
                  mqttService.mqttClient.connectionStatus.state ==
                      MqttConnectionState.connected
              ? Home()
              : FutureBuilder<MqttClient>(
                  future: mqttService.connect(),
                  builder: (BuildContext context,
                      AsyncSnapshot<MqttClient> snapshot) {
                    if (snapshot.connectionState == ConnectionState.done) {
                      if (snapshot.hasData) {
                        return Home();
                      }
                    }
                    return SplashScreen();
                  },
                ),
        );
      }
    });
  }
}
