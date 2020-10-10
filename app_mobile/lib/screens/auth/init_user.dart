import 'package:flutter/material.dart';
import 'package:homeflow/services/firebase.dart';

class InitUser extends StatefulWidget {
  @override
  _InitUserState createState() => _InitUserState();
}

class _InitUserState extends State<InitUser> {
  final _formKey = GlobalKey<FormState>();
  String username;

  void onSubmit(BuildContext context) async {
    if (_formKey.currentState.validate()) {
      _formKey.currentState.save();

      await FirebaseService.initializeNewUser()
          .call({'username': username}).catchError((e) {
        Scaffold.of(context)
            .showSnackBar(SnackBar(content: Text('Error occured! $e')));
      }).then((value) {
        Scaffold.of(context)
            .showSnackBar(const SnackBar(content: Text('Success!')));
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Initialize user"),
      ),
      body: Builder(builder: (context) {
        return Container(
          margin: const EdgeInsets.symmetric(horizontal: 20),
          child: Form(
            key: _formKey,
            child: Column(
              children: [
                TextFormField(
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
                RaisedButton(
                  onPressed: () => onSubmit(context),
                  child: const Text('Submit'),
                )
              ],
            ),
          ),
        );
      }),
    );
  }
}
