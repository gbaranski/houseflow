import 'package:houseflow/screens/splash_screen/splash_screen.dart';
import 'package:houseflow/screens/wrapper.dart';
import 'package:houseflow/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:provider/provider.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(App());
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
              return const Text("Error");
            }
            if (snapshot.connectionState == ConnectionState.done) {
              final AuthService authService = AuthService();

              return StreamProvider<User>.value(
                  value: authService.user,
                  initialData: null,
                  child: ChangeNotifierProvider<AuthService>.value(
                      value: authService, child: Wrapper()));
            }

            return SplashScreen();
          },
        ));
  }
}
