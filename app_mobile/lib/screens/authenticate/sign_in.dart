import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/shared/constants.dart';
import 'package:app_mobile/shared/loading.dart';
import 'package:flutter/material.dart';

class SignIn extends StatefulWidget {
  final Function toggleView;

  SignIn({this.toggleView});

  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  final AuthService _authService = AuthService();
  final _formKey = GlobalKey<FormState>();
  bool loading = false;

  String email = '';
  String password = '';

  @override
  Widget build(BuildContext context) {
    return loading
        ? Loading()
        : Scaffold(
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
                    label: Text('Register'))
              ],
            ),
            body: Builder(builder: (BuildContext context) {
              return Container(
                  padding: EdgeInsets.symmetric(
                    vertical: 20,
                    horizontal: 50,
                  ),
                  child: Column(children: [
                    RaisedButton(
                      child: Text("Sign in anonymously"),
                      onPressed: () async {
                        dynamic result = await _authService.signInAnon();
                        if (result == null) {
                          print('Error signing in');
                        } else {
                          print('Signed in');
                          print(result.uid);
                        }
                      },
                    ),
                    Form(
                      key: _formKey,
                      child: Column(
                        children: <Widget>[
                          SizedBox(height: 20.0),
                          TextFormField(
                            decoration:
                                textInputDecoration.copyWith(hintText: "Email"),
                            validator: (val) =>
                                val.isEmpty ? "Enter an email" : null,
                            onChanged: (value) {
                              setState(() {
                                email = value;
                              });
                            },
                          ),
                          SizedBox(height: 20.0),
                          TextFormField(
                            decoration: textInputDecoration.copyWith(
                                hintText: "Password"),
                            obscureText: true,
                            validator: (val) => val.length < 6
                                ? "Enter password 6+ chars long"
                                : null,
                            onChanged: (value) {
                              setState(() {
                                password = value;
                              });
                            },
                          ),
                          SizedBox(height: 20.0),
                          RaisedButton(
                            color: Colors.pink[400],
                            child: Text(
                              'Sign in',
                              style: TextStyle(color: Colors.white),
                            ),
                            onPressed: () async {
                              if (_formKey.currentState.validate()) {
                                setState(() {
                                  loading = true;
                                });
                                try {
                                  await _authService.signInWithEmailAndPassword(
                                      email, password);
                                } catch (e) {
                                  final snackBar = SnackBar(
                                    content: Text(e.toString()),
                                  );
                                  Scaffold.of(context).showSnackBar(snackBar);
                                } finally {
                                  setState(() {
                                    loading = false;
                                  });
                                }
                              }
                            },
                          ),
                        ],
                      ),
                    )
                  ]));
            }));
  }
}
