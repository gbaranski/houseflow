import 'package:device_preview/device_preview.dart';
import 'package:flutter/foundation.dart';
import 'package:houseflow/screens/splash_screen/splash_screen.dart';
import 'package:houseflow/screens/wrapper.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:provider/provider.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  return runApp(App());
  runApp(DevicePreview(
    enabled: !kReleaseMode,
    builder: (context) => App(),
  ));
}

class App extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        debugShowCheckedModeBanner: false,
        navigatorObservers: [FirebaseService.observer],
        theme:
            ThemeData(fontFamily: 'OpenSans', accentColor: Color(0xFF0096c7)),
        home: FutureBuilder(
          future: Firebase.initializeApp(),
          builder: (context, snapshot) {
            // return SplashScreen();
            if (snapshot.hasError) {
              print(snapshot.error);
              // TODO: Add error page
              return const Text("Error");
            }
            if (snapshot.connectionState == ConnectionState.done) {
              return ChangeNotifierProvider<MqttService>(
                  create: (_) => new MqttService(), child: Wrapper());
            }

            return SplashScreen();
          },
        ));
  }
}
