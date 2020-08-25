import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/shared/constants.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class MyProfile extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (context, model, child) {
        return Container(
          child: Column(
            children: [
              Text("UID: ${model.firebaseUser.uid}"),
              RaisedButton(
                color: LayoutBlueColor1,
                textColor: Colors.white,
                child: Text("Log out"),
                onPressed: model.signOut,
              )
            ],
          ),
        );
      },
    );
  }
}
