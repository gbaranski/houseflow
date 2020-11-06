import 'package:houseflow/shared/constants.dart';
import 'package:flutter/material.dart';

class LoginForm extends StatefulWidget {
  final Function(String email, String password) onSubmit;
  final Function onSuccess;
  final String submitMessage;

  const LoginForm(
      {@required this.onSubmit, @required this.submitMessage, this.onSuccess});

  @override
  _LoginFormState createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  final formKey = GlobalKey<FormState>();

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

  void submitForm(BuildContext context) async {
    if (formKey.currentState.validate()) {
      formKey.currentState.save();
      try {
        await widget.onSubmit(email, password);
        final snackBar = SnackBar(
          content: Text("Success!"),
        );
        Scaffold.of(context).showSnackBar(snackBar);
        widget.onSuccess?.call();
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
    return Form(
      key: formKey,
      child: Container(
          child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          SizedBox(height: 20.0),
          TextFormField(
            keyboardType: TextInputType.emailAddress,
            focusNode: emailFocusNode,
            style: textInputTextStyle,
            textInputAction: TextInputAction.next,
            onFieldSubmitted: (term) {
              emailFocusNode.unfocus();
              passwordFocusNode.requestFocus();
            },
            decoration: textInputDecoration.copyWith(labelText: "Email"),
            validator: (val) => val.isEmpty ? "Enter an email" : null,
            onSaved: (value) {
              setState(() {
                email = value;
              });
            },
          ),
          const SizedBox(height: 20.0),
          TextFormField(
            keyboardType: TextInputType.text,
            focusNode: passwordFocusNode,
            textInputAction: TextInputAction.done,
            onFieldSubmitted: (term) {
              submitForm(context);
            },
            style: textInputTextStyle,
            decoration: textInputDecoration.copyWith(labelText: "Password"),
            onSaved: (value) {
              setState(() {
                password = value;
              });
            },
            validator: (val) =>
                val.length < 6 ? "Enter password 8+ chars long" : null,
            obscureText: true,
          ),
          const SizedBox(height: 25),
          ButtonTheme(
            height: 60,
            shape: const RoundedRectangleBorder(
                borderRadius:
                    const BorderRadius.all(Radius.elliptical(30, 30))),
            child: RaisedButton(
              color: LayoutBlueColor1,
              child: Text(
                widget.submitMessage,
                style: const TextStyle(color: Colors.white, fontSize: 22),
              ),
              onPressed: () => submitForm(context),
            ),
          ),
        ],
      )),
    );
  }
}
