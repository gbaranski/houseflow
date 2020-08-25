import 'package:app_mobile/shared/constants.dart';
import 'package:flutter/material.dart';

class LoginForm extends StatefulWidget {
  final Function onSubmit;
  final Function onSuccess;
  final formKey;
  final String submitMessage;
  final String successMessage;

  const LoginForm(
      {@required this.onSubmit,
      @required this.formKey,
      @required this.submitMessage,
      this.successMessage,
      this.onSuccess});

  @override
  _LoginFormState createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  FocusNode emailFocusNode;
  FocusNode passwordFocusNode;

  @override
  void initState() {
    super.initState();
    emailFocusNode = FocusNode();
    passwordFocusNode = FocusNode();
  }

  @override
  void dispose() {
    emailFocusNode.dispose();
    passwordFocusNode.dispose();
    super.dispose();
  }

  String email = '';
  String password = '';

  void submitForm() async {
    if (widget.formKey.currentState.validate()) {
      try {
        await widget.onSubmit(email, password);
        if (widget.successMessage != null) {
          final snackBar = SnackBar(
            content: Text(widget.successMessage),
          );
          Scaffold.of(context).showSnackBar(snackBar);
        }
        widget.onSuccess();
      } catch (e) {
        final snackBar = SnackBar(
          content: Text(e.toString()),
        );
        Scaffold.of(context).showSnackBar(snackBar);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
        child: Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        SizedBox(height: 20.0),
        TextFormField(
          focusNode: emailFocusNode,
          style: textInputTextStyle,
          textInputAction: TextInputAction.next,
          onFieldSubmitted: (term) {
            emailFocusNode.unfocus();
            passwordFocusNode.requestFocus();
          },
          decoration: textInputDecoration.copyWith(labelText: "Email"),
          validator: (val) => val.isEmpty ? "Enter an email" : null,
          onChanged: (value) {
            setState(() {
              email = value;
            });
          },
        ),
        SizedBox(height: 20.0),
        TextFormField(
          focusNode: passwordFocusNode,
          textInputAction: TextInputAction.done,
          onFieldSubmitted: (term) {
            submitForm();
          },
          style: textInputTextStyle,
          decoration: textInputDecoration.copyWith(labelText: "Password"),
          onChanged: (value) {
            setState(() {
              password = value;
            });
          },
          validator: (val) =>
              val.length < 6 ? "Enter password 8+ chars long" : null,
          obscureText: true,
        ),
        SizedBox(height: 25),
        ButtonTheme(
          height: 60,
          shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.all(Radius.elliptical(30, 30))),
          child: RaisedButton(
            color: LayoutBlueColor1,
            child: Text(
              widget.submitMessage,
              style: TextStyle(color: Colors.white, fontSize: 22),
            ),
            onPressed: submitForm,
          ),
        ),
      ],
    ));
  }
}
