import 'package:homeflow/services/auth.dart';
import 'package:homeflow/shared/login_form.dart';
import 'package:flutter/material.dart';

class Register extends StatefulWidget {
  @override
  _RegisterState createState() => _RegisterState();
}

class _RegisterState extends State<Register> {
  final AuthService _authService = AuthService();
  final _formKey = GlobalKey<FormState>();

  String email = '';
  String password = '';

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
                    onSubmit: _authService.registerWithEmailAndPassword,
                    submitMessage: 'REGISTER',
                    onSuccess: () => Navigator.pop(context),
                  ),
                  const SizedBox(
                    height: 50,
                  ),
                ])));
  }
}
