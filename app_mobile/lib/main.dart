import 'package:flutter/foundation.dart';
import 'package:houseflow/screens/error_screen/error_screen.dart';
import 'package:houseflow/screens/splash_screen/splash_screen.dart';
import 'package:houseflow/screens/wrapper.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/auth.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:provider/provider.dart';
import 'package:flutter/services.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  return runApp(App());

  // runApp(DevicePreview(
  //   enabled: kDebugMode,
  //   builder: (context) => App(),
  // ));
}

class App extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        title: 'Houseflow',
        debugShowCheckedModeBanner: false,
        navigatorObservers: [FirebaseService.observer],
        theme: ThemeData(
            fontFamily: 'OpenSans',
            accentColor: Color(0xFF0096c7),
            appBarTheme: AppBarTheme(brightness: Brightness.light)),
        home: FutureBuilder(
          future: Firebase.initializeApp(),
          builder: (context, snapshot) {
            if (snapshot.hasError) {
              print(snapshot.error);
              FirebaseService.crashlytics
                  .recordError(snapshot.error, StackTrace.current);
              return ErrorScreen(
                reason: 'Could not initialize Firebase',
              );
            }
            if (snapshot.connectionState == ConnectionState.done) {
              FlutterError.onError =
                  FirebaseService.crashlytics.recordFlutterError;
              return ChangeNotifierProvider<MqttService>(
                  create: (_) => new MqttService(),
                  child: ChangeNotifierProvider<AuthService>(
                      create: (_) => new AuthService(), child: Wrapper()));
            }

            return SplashScreen();
          },
        ));
  }
}
