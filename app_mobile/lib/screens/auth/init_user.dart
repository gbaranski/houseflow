import 'package:firebase_auth/firebase_auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:rounded_loading_button/rounded_loading_button.dart';

class InitUser extends StatefulWidget {
  final User currentUser;
  InitUser({@required this.currentUser});

  @override
  _InitUserState createState() => _InitUserState();
}

class _InitUserState extends State<InitUser> {
  final _formKey = GlobalKey<FormState>();
  String username;

  final RoundedLoadingButtonController _btnController =
      new RoundedLoadingButtonController();

  void onSubmit(BuildContext context) async {
    if (!_formKey.currentState.validate()) return _btnController.error();

    try {
      _formKey.currentState.save();
      await FirebaseService.initializeNewUser().call({'username': username});
      Scaffold.of(context)
          .showSnackBar(const SnackBar(content: Text('Success!')));
      _btnController.success();
    } catch (e) {
      Scaffold.of(context)
          .showSnackBar(SnackBar(content: Text('Error occured! $e')));
      print("Error occured when intializing user $e");
      _btnController.error();
    }
  }

  void onFormValueChange() {
    _btnController.reset();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Builder(builder: (context) {
        return Container(
          margin: const EdgeInsets.symmetric(horizontal: 20, vertical: 80),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                "Introduce yourself",
                style: TextStyle(fontSize: 25, fontWeight: FontWeight.w600),
              ),
              Form(
                key: _formKey,
                onChanged: onFormValueChange,
                child: Column(
                  children: [
                    TextFormField(
                      initialValue: widget.currentUser.displayName,
                      keyboardType: TextInputType.name,
                      onSaved: (String value) {
                        username = value;
                      },
                      decoration: const InputDecoration(
                          icon: const Icon(Icons.person),
                          hintText: "Enter username",
                          labelText: "Username"),
                      // The validator receives the text that the user has entered.
                      validator: (value) {
                        if (value.isEmpty) {
                          return 'Please enter username';
                        }
                        return null;
                      },
                    ),
                    SizedBox(
                      height: 20,
                    ),
                    RoundedLoadingButton(
                      child: Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Text('Submit',
                                style: TextStyle(
                                    color: Colors.white, fontSize: 16)),
                            SizedBox(
                              width: 5,
                            ),
                            Icon(Icons.send, color: Colors.white),
                          ]),
                      controller: _btnController,
                      onPressed: () => onSubmit(context),
                    )
                  ],
                ),
              )
            ],
          ),
        );
      }),
    );
  }
}
