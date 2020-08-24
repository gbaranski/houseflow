import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/shared/login_form.dart';
import 'package:flutter/material.dart';

class Register extends StatefulWidget {
  final Function toggleView;

  Register({this.toggleView});

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
        backgroundColor: Colors.brown[100],
        appBar: AppBar(
          backgroundColor: Colors.brown[400],
          elevation: 0.0,
          title: Text("Sign in to Control Home"),
          actions: [
            FlatButton.icon(
                onPressed: () {
                  widget.toggleView();
                },
                icon: Icon(Icons.person),
                label: Text('Sign in'))
          ],
        ),
        body: Builder(builder: (BuildContext context) {
          return Container(
            padding: EdgeInsets.symmetric(
              vertical: 20,
              horizontal: 50,
            ),
            child: Column(children: [
              Form(
                  key: _formKey,
                  child: LoginForm(
                    onSubmit: _authService.registerWithEmailAndPassword,
                    submitMessage: 'Register',
                    formKey: _formKey,
                  ))
            ]),
          );
        }));
  }
}
