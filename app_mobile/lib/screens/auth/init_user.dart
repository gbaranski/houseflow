import 'package:flutter/material.dart';
import 'package:homeflow/services/firebase.dart';

class InitUser extends StatefulWidget {
  @override
  _InitUserState createState() => _InitUserState();
}

class _InitUserState extends State<InitUser> {
  final _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Initialize user"),
      ),
      body: Builder(builder: (context) {
        return Container(
          margin: const EdgeInsets.symmetric(horizontal: 20),
          child: Form(
            key: _formKey,
            child: Column(
              children: [
                TextFormField(
                  decoration: const InputDecoration(
                      icon: Icon(Icons.person),
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
                  onPressed: () async {
                    // Validate returns true if the form is valid, otherwise false.
                    if (_formKey.currentState.validate()) {
                      // If the form is valid, display a snackbar. In the real world,
                      // you'd often call a server or save the information in a database.
                      await FirebaseService.initializeNewUser().call(
                          {'username': 'someRandomUsername'}).catchError((e) {
                        Scaffold.of(context).showSnackBar(
                            SnackBar(content: Text('Error occured! $e')));
                      }).then((value) {
                        Scaffold.of(context)
                            .showSnackBar(SnackBar(content: Text('Success!')));
                      });
                    }
                  },
                  child: Text('Submit'),
                )
              ],
            ),
          ),
        );
      }),
    );
  }
}
