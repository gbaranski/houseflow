import 'package:homeflow/screens/wrapper.dart';
import 'package:homeflow/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

void main() => runApp(App());

class App extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        theme: ThemeData(fontFamily: 'OpenSans'),
        home: FutureBuilder(
          future: Firebase.initializeApp(),
          builder: (context, snapshot) {
            if (snapshot.hasError) {
              print(snapshot.error);
              return Text("Error");
            }
            if (snapshot.connectionState == ConnectionState.done) {
              final AuthService authService = AuthService();
              return StreamProvider<User>.value(
                  value: authService.user,
                  child: ChangeNotifierProvider<AuthService>.value(
                      value: authService, child: Wrapper()));
            }

            return CircularProgressIndicator();
          },
        ));
  }
}
