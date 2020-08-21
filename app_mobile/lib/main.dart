import 'package:flutter/material.dart';
import 'home.dart';


void main() => runApp(App());


class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Welcome to Flutter das',
      home: Home()
    );
  }
}
