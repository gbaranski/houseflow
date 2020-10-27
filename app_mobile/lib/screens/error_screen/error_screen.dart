import 'package:flutter/material.dart';
import 'package:houseflow/screens/support/help_screen.dart';

class ErrorScreen extends StatelessWidget {
  final String reason;
  ErrorScreen({this.reason = 'Some error occurred'});

  void navigateToHelpScreen(BuildContext context) {
    Navigator.push(
        context,
        MaterialPageRoute(
            settings: const RouteSettings(name: 'Support'),
            builder: (context) => HelpScreen()));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Icon(
              Icons.error,
              size: 128,
              color: Colors.red,
            ),
            GestureDetector(
              onTap: () => navigateToHelpScreen(context),
              child: Column(
                children: [
                  Text(
                    reason,
                    style: TextStyle(color: Colors.black54, fontSize: 22),
                  ),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        "If problem persists, please ",
                        style: TextStyle(color: Colors.black45, fontSize: 15),
                      ),
                      Text(
                        "contact us",
                        style: TextStyle(color: Colors.blue, fontSize: 15),
                      ),
                    ],
                  )
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
