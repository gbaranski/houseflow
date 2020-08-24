import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/shared/loading.dart';
import 'package:app_mobile/shared/login_form.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

class SignIn extends StatefulWidget {
  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  final AuthService _authService = AuthService();
  final _formKey = GlobalKey<FormState>();

  String email = '';
  String password = '';

  // TODO Move it to constants
  List<Color> _gradientColors = [
    Colors.blueGrey,
    Colors.blue.shade800,
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: LayoutBuilder(
        builder: (BuildContext context, BoxConstraints viewportConstraints) {
      return SingleChildScrollView(
        child: ConstrainedBox(
          constraints: BoxConstraints(
            minHeight: viewportConstraints.maxHeight,
            minWidth: viewportConstraints.minWidth,
          ),
          child: IntrinsicHeight(
            child: Container(
              padding: const EdgeInsets.symmetric(
                horizontal: 20,
              ),
              child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    SizedBox(
                      height: 50,
                    ),
                    // TODO move to constants
                    SvgPicture.asset(
                      'assets/images/logo.svg',
                      semanticsLabel: "Logo",
                      height: 200,
                    ),
                    SizedBox(height: 30),
//                      RaisedButton(
//                        child: Text("Sign in anonymously"),
//                        onPressed: () async {
//                          dynamic result = await _authService.signInAnon();
//                          if (result == null) {
//                            print('Error signing in');
//                          } else {
//                            print('Signed in');
//                            print(result.uid);
//                          }
//                        },
//                      ),
                    Form(
                        key: _formKey,
                        child: Column(children: <Widget>[
                          LoginForm(
                            onSubmit: _authService.signInWithEmailAndPassword,
                            submitMessage: 'LOG IN',
                            formKey: _formKey,
                          ),
                        ])),
                    SizedBox(
                      height: 15,
                    ),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Column(
                          children: [
                            Row(
                              children: [
                                Text(
                                  "Reset",
                                  style: TextStyle(
                                    color: Colors.blueAccent[200],
                                    fontWeight: FontWeight.w700,
                                  ),
                                ),
                                Text(
                                  " password!",
                                ),
                              ],
                            )
                          ],
                        ),
                        Column(
                          children: [
                            Row(children: [
                              Text("New user? "),
                              Text(
                                "SIGN UP",
                                style: TextStyle(
                                  color: Colors.blueAccent[200],
                                  fontWeight: FontWeight.w700,
                                ),
                              ),
                            ])
                          ],
                        )
                      ],
                    ),

                    SizedBox(
                      height: 50,
                    ),
                  ]),
            ),
          ),
        ),
      );
    }));
  }
}
