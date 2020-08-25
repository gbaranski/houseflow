import 'package:app_mobile/services/auth.dart';
import 'package:flutter/material.dart';

class Settings extends StatelessWidget {
  final AuthService _authService = AuthService();

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text("Settings"),
    );
  }
}
