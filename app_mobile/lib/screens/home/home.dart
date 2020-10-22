import 'package:houseflow/screens/dashboard/dashboard.dart';
import 'package:houseflow/screens/my_profile/my_profile.dart';
import 'package:houseflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/shared/profile_image.dart';
import 'package:provider/provider.dart';

class Home extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (BuildContext context, authModel, child) {
        return Scaffold(
            backgroundColor: Colors.white,
            appBar: PreferredSize(
              preferredSize: Size.fromHeight(80),
              child: Container(
                margin: EdgeInsets.only(top: 15),
                child: AppBar(
                  backgroundColor: Colors.white,
                  elevation: 0,
                  actions: [
                    GestureDetector(
                      onTap: () => Navigator.push(
                          context,
                          MaterialPageRoute(
                              builder: (context) => MyProfile(
                                  firebaseUser: authModel.firebaseUser,
                                  currentUser: authModel.currentUser,
                                  signOut: authModel.signOut))),
                      child: ProfileImage(
                        size: 38,
                        imageUrl: authModel.currentUser.photoURL,
                      ),
                    ),
                    SizedBox(
                      width: 10,
                    ),
                  ],
                  title: Text(
                    "Hi, ${authModel.firebaseUser.username.split(' ')[0]}!",
                    style: TextStyle(
                        color: Colors.black.withAlpha(160),
                        fontSize: 20,
                        fontWeight: FontWeight.w600),
                  ),
                ),
              ),
            ),
            body: Dashboard());
      },
    );
  }
}
