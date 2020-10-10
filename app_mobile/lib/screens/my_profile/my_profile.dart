import 'package:firebase_auth/firebase_auth.dart' as firebase;
import 'package:homeflow/models/user.dart';
import 'package:homeflow/shared/constants.dart';
import 'package:homeflow/shared/profile_image.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class MyProfile extends StatelessWidget {
  final FirebaseUser firebaseUser;
  final firebase.User currentUser;
  final Future Function() signOut;

  MyProfile({
    @required this.firebaseUser,
    @required this.currentUser,
    @required this.signOut,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Builder(
        builder: (BuildContext context) => Container(
          padding: const EdgeInsets.only(top: 20),
          alignment: Alignment.topCenter,
          child: Column(
            children: [
              ProfileImage(
                imageUrl: currentUser.photoURL,
              ),
              const SizedBox(
                height: 10,
              ),
              GestureDetector(
                onLongPress: () {
                  Clipboard.setData(ClipboardData(text: currentUser.uid))
                      .then((_) {
                    HapticFeedback.vibrate();
                    const snackBar = SnackBar(
                      content: Text("UID copied to clipboard"),
                    );
                    Scaffold.of(context).showSnackBar(snackBar);
                  });
                },
                child: Text(
                  firebaseUser.username,
                  style: const TextStyle(
                    fontSize: 26,
                  ),
                ),
              ),
              RaisedButton(
                color: LayoutBlueColor1,
                textColor: Colors.white,
                child: Text("Log out"),
                onPressed: () {
                  signOut().then((value) => Navigator.pop(context));
                },
              )
            ],
          ),
        ),
      ),
    );
  }
}
