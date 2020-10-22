import 'package:houseflow/screens/auth/register.dart';
import 'package:houseflow/services/auth.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/shared/login_form.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

class SignIn extends StatefulWidget {
  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  final AuthService _authService = AuthService();

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
                    const SizedBox(
                      height: 50,
                    ),
                    SvgPicture.asset(
                      logoDirectory,
                      semanticsLabel: "Logo",
                      height: 200,
                    ),
                    const SizedBox(height: 10),
                    LoginForm(
                      onSubmit: _authService.signInWithEmailAndPassword,
                      submitMessage: 'LOG IN',
                    ),
                    const SizedBox(
                      height: 15,
                    ),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Column(
                          children: [
                            Row(
                              children: [
                                const Text(
                                  "Reset",
                                  style: const TextStyle(
                                    color: LayoutBlueColor1,
                                    fontWeight: FontWeight.w700,
                                  ),
                                ),
                                const Text(
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
                                        settings:
                                            RouteSettings(name: 'Register'),
                                        builder: (context) => Register()));
                              },
                              child: Row(children: [
                                const Text("New user? "),
                                const Text(
                                  "SIGN UP",
                                  style: const TextStyle(
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
                          style: const TextStyle(
                              color: LayoutBlueColor1,
                              fontSize: 18,
                              fontWeight: FontWeight.w800),
                        ),
                        const SizedBox(
                          height: 15,
                        ),
                        Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            CircleAvatar(
                              radius: 25,
                              backgroundColor: LayoutBlueColor1,
                              child: IconButton(
                                icon: const Icon(
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
                            const SizedBox(
                              width: 10,
                            ),
                            CircleAvatar(
                              radius: 25,
                              backgroundColor: Colors.black,
                              child: IconButton(
                                icon: const Icon(
                                  MdiIcons.apple,
                                  color: Colors.white,
                                ),
                                onPressed: () async {
                                  try {
                                    await _authService.signInWithApple();
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
                            const SizedBox(
                              width: 10,
                            ),
                            const CircleAvatar(
                              radius: 25,
                              backgroundColor: Colors.black45,
                              child: const IconButton(
                                icon: const Icon(
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
                    const SizedBox(
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
