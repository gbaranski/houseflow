import 'package:app_mobile/services/auth.dart';
import 'package:app_mobile/shared/constants.dart';
import 'package:app_mobile/shared/profile_image.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';

class MyProfile extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (_context, model, child) {
        return Container(
          padding: const EdgeInsets.only(top: 20),
          alignment: Alignment.topCenter,
          child: Column(
            children: [
              ProfileImage(
                imageUrl: model.currentUser.photoURL,
              ),
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
