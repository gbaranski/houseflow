import 'package:homeflow/shared/constants.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class SplashScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        alignment: Alignment.center,
        decoration: const BoxDecoration(
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
            const SizedBox(
              height: 10,
            ),
            const Text(
              "Homeflow",
              style: const TextStyle(
                fontSize: 32,
              ),
            )
          ],
        ),
      ),
    );
  }
}
