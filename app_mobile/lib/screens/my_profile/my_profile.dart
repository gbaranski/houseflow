import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/shared/constants.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';

class MyProfile extends StatelessWidget {
  Widget noProfileImage(BuildContext context) {
    return SizedBox(
      height: 200,
      width: 200,
      child: DecoratedBox(
        decoration: const BoxDecoration(
            color: Colors.black26,
            borderRadius: BorderRadius.all(Radius.circular(100))),
        child: Icon(
          Icons.person_outline,
          color: Colors.white,
          size: 150,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (_context, model, child) {
        return Container(
          padding: const EdgeInsets.only(top: 20),
          alignment: Alignment.topCenter,
          child: Column(
            children: [
              model.currentUser.photoURL == null
                  ? noProfileImage(context)
                  : Image.network(model.currentUser.photoURL),
              SizedBox(
                height: 10,
              ),
              GestureDetector(
                onLongPress: () {
                  Clipboard.setData(ClipboardData(text: model.currentUser.uid))
                      .then((_) {
                    HapticFeedback.vibrate();
                    final snackBar = SnackBar(
                      content: Text("UID copied to clipboard"),
                    );
                    Scaffold.of(context).showSnackBar(snackBar);
                  });
                },
                child: Text(
                  model.currentUser.displayName,
                  style: const TextStyle(
                    fontSize: 26,
                  ),
                ),
              ),
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
