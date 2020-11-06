import 'package:houseflow/services/auth.dart';
import 'package:houseflow/shared/login_form.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class Register extends StatelessWidget {
  final Function(String email, String password) onSubmit;
  Register({@required this.onSubmit});

  @override
  Widget build(BuildContext context) {
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
                    onSubmit: onSubmit,
                    submitMessage: 'REGISTER',
                    onSuccess: () => Navigator.pop(context),
                  ),
                  const SizedBox(
                    height: 50,
                  ),
                ])));
  }
}
