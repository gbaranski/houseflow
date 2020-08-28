import 'package:control_home/screens/auth/register.dart';
import 'package:control_home/services/auth.dart';
import 'package:control_home/shared/constants.dart';
import 'package:control_home/shared/login_form.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

class SignIn extends StatefulWidget {
  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  final AuthService _authService = AuthService();
  final _formKey = GlobalKey<FormState>();

  String email = '';
  String password = '';

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
                                    color: LayoutBlueColor1,
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
                            GestureDetector(
                              onTap: () {
                                Navigator.push(
                                    context,
                                    MaterialPageRoute(
                                        builder: (context) => Register()));
                              },
                              child: Row(children: [
                                Text("New user? "),
                                Text(
                                  "SIGN UP",
                                  style: TextStyle(
                                    color: LayoutBlueColor1,
                                    fontWeight: FontWeight.w700,
                                  ),
                                ),
                              ]),
                            )
                          ],
                        )
                      ],
                    ),
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        Text(
                          "OR",
                          style: TextStyle(
                              color: LayoutBlueColor1,
                              fontSize: 18,
                              fontWeight: FontWeight.w800),
                        ),
                        SizedBox(
                          height: 15,
                        ),
                        Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            CircleAvatar(
                              radius: 25,
                              backgroundColor: LayoutBlueColor1,
                              child: IconButton(
                                icon: Icon(
                                  MdiIcons.google,
                                  color: Colors.white,
                                ),
                                onPressed: () async {
                                  try {
                                    await _authService.signInWithGoogle();
                                  } catch (e) {
                                    print(e.toString());
                                    final snackBar = SnackBar(
                                      content: Text(e.toString()),
                                    );
                                    Scaffold.of(context).showSnackBar(snackBar);
                                  }
                                },
                              ),
                            ),
                            SizedBox(
                              width: 10,
                            ),
                            CircleAvatar(
                              radius: 25,
                              backgroundColor: Colors.black45,
                              child: IconButton(
                                icon: Icon(
                                  MdiIcons.incognito,
                                  color: Colors.white,
                                ),
                                onPressed: null,
                              ),
                            )
                          ],
                        ),
                      ],
                    ),

                    SizedBox(
                      height: 20,
                    ),
                  ]),
            ),
          ),
        ),
      );
    }));
  }
}
