import 'package:control_home/shared/constants.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class SplashScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: Colors.white,
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            SvgPicture.asset(
              logoDirectory,
              semanticsLabel: "Logo",
              height: 200,
              alignment: Alignment.center,
            ),
            SizedBox(
              height: 10,
            ),
            Text(
              "Control Home",
              style: TextStyle(
                fontSize: 32,
              ),
            )
          ],
        ),
      ),
    );
  }
}
