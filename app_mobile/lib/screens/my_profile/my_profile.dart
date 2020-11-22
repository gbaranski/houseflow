import 'package:firebase_auth/firebase_auth.dart' as firebase;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/user.dart';
import 'package:houseflow/screens/additional_view/index.dart';
import 'package:houseflow/shared/profile_image.dart';

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
    return AdditionalView(
        body: Builder(
            builder: (BuildContext context) => Column(
                  children: [
                    ProfileImage(
                      imageUrl: currentUser.photoURL,
                      size: 100,
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
                            fontSize: 28, fontWeight: FontWeight.w500),
                      ),
                    ),
                    RaisedButton(
                      color: Colors.blueGrey.shade400,
                      textColor: Colors.white,
                      child: Text("Log out"),
                      onPressed: () {
                        signOut().then((value) => Navigator.pop(context));
                      },
                    )
                  ],
                )));
  }
}
