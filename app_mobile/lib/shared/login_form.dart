import 'package:app_mobile/shared/constants.dart';
import 'package:flutter/material.dart';

class LoginForm extends StatefulWidget {
  final Function onSubmit;
  final formKey;
  final submitMessage;

  const LoginForm(
      {@required this.onSubmit,
      @required this.formKey,
      @required this.submitMessage});

  @override
  _LoginFormState createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  String email = '';
  String password = '';

  @override
  Widget build(BuildContext context) {
    return Container(
        child: Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        SizedBox(height: 20.0),
        TextFormField(
          decoration: textInputDecoration.copyWith(hintText: "Email"),
          validator: (val) => val.isEmpty ? "Enter an email" : null,
          onChanged: (value) {
            setState(() {
              email = value;
            });
          },
        ),
        SizedBox(height: 10.0),
        TextFormField(
          decoration: textInputDecoration.copyWith(hintText: "Password"),
          obscureText: true,
          validator: (val) =>
              val.length < 6 ? "Enter password 6+ chars long" : null,
          onChanged: (value) {
            setState(() {
              password = value;
            });
          },
        ),
        SizedBox(height: 10.0),
        RaisedButton(
          color: Colors.pink[400],
          child: Text(
            widget.submitMessage,
            style: TextStyle(color: Colors.white),
          ),
          onPressed: () async {
            if (widget.formKey.currentState.validate()) {
              try {
                await widget.onSubmit(email, password);
              } catch (e) {
                final snackBar = SnackBar(
                  content: Text(e.toString()),
                );
                Scaffold.of(context).showSnackBar(snackBar);
              }
            }
          },
        ),
      ],
    ));
  }
}
