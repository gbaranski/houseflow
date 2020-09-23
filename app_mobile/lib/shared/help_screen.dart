import 'package:homeflow/shared/constants.dart';
import 'package:flutter/material.dart';

class HelpScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Help"),
        backgroundColor: LayoutBlueColor1,
      ),
      body: Container(
        margin: const EdgeInsets.symmetric(vertical: 20, horizontal: 20),
        alignment: Alignment.topCenter,
        child: Text(
            "Looks like you've got a problem\nPlease send an email to gbaranski19@gmail.com"),
      ),
    );
  }
}
