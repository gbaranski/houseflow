import 'package:app_mobile/screens/wrapper.dart';
import 'package:app_mobile/services/auth.dart';
import 'package:firebase_auth/firebase_auth.dart';
import 'package:firebase_core/firebase_core.dart';
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:provider/provider.dart';

void main() => runApp(App());

class App extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        theme: ThemeData().copyWith(
            textTheme:
                GoogleFonts.openSansTextTheme(Theme.of(context).textTheme)),
        home: FutureBuilder(
          future: Firebase.initializeApp(),
          builder: (context, snapshot) {
            if (snapshot.hasError) {
              print(snapshot.error);
              return Text("Error");
            }
            if (snapshot.connectionState == ConnectionState.done) {
              return StreamProvider<User>.value(
                  value: AuthService().user, child: Wrapper());
            }

            return CircularProgressIndicator();
          },
        ));
  }
}
