import 'package:houseflow/screens/dashboard/dashboard.dart';
import 'package:flutter/material.dart';

class Home extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
        backgroundColor: Colors.white,
        // child: SliverAppBar(
        //   backgroundColor: Colors.white,
        //   elevation: 0,
        //   actions: [
        //     GestureDetector(
        //       onTap: () => Navigator.push(
        //           context,
        //           MaterialPageRoute(
        //               settings: const RouteSettings(name: 'My profile'),
        //               builder: (context) => MyProfile(
        //                   firebaseUser: authModel.firebaseUser,
        //                   currentUser: authModel.currentUser,
        //                   signOut: authModel.signOut))),
        //       child: ProfileImage(
        //         size: 38,
        //         imageUrl: authModel.currentUser.photoURL,
        //       ),
        //     ),
        //     SizedBox(
        //       width: 10,
        //     ),
        //   ],
        //   title: Text(
        //     "Hi, ${authModel.firebaseUser.username.split(' ')[0]}!",
        //     style: TextStyle(
        //         color: Colors.black.withAlpha(160),
        //         fontSize: 20,
        //         fontWeight: FontWeight.w600),
        //   ),
        // ),
        body: Dashboard());
  }
}
