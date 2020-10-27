import 'package:houseflow/services/auth.dart';
import 'package:houseflow/shared/login_form.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Register extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final AuthService authService =
        Provider.of<AuthService>(context, listen: false);
    return Scaffold(
        appBar: AppBar(
          title: const Text("Register"),
        ),
        body: Container(
            padding: const EdgeInsets.symmetric(
              horizontal: 20,
            ),
            child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const SizedBox(height: 10),
                  LoginForm(
                    onSubmit: authService.registerWithEmailAndPassword,
                    submitMessage: 'REGISTER',
                    onSuccess: () => Navigator.pop(context),
                  ),
                  const SizedBox(
                    height: 50,
                  ),
                ])));
  }
}
