import 'package:flutter_svg/svg.dart';
import 'package:houseflow/screens/dashboard/dashboard.dart';
import 'package:houseflow/screens/my_profile/my_profile.dart';
import 'package:houseflow/screens/settings/settings.dart';
import 'package:houseflow/services/auth.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/shared/profile_image.dart';
import 'package:provider/provider.dart';

class Home extends StatefulWidget {
  @override
  _HomeState createState() => _HomeState();
}

class _HomeState extends State<Home> {
  static final List<Widget> _navPages = <Widget>[
    Dashboard(),
    Settings(),
  ];

  static const List<BottomNavigationBarItem> _navItems = [
    const BottomNavigationBarItem(
        icon: const Icon(
          Icons.home,
          size: 28,
        ),
        label: 'Home'),
    const BottomNavigationBarItem(
        icon: const Icon(
          Icons.settings,
          size: 28,
        ),
        label: 'Settings'),
  ];

  int _currentIndex = 0;

  void onNavItemTap(int index) {
    setState(() {
      _currentIndex = index;
    });
  }

  BottomNavigationBar navigation(BuildContext context) {
    return BottomNavigationBar(
      selectedFontSize: 14,
      type: BottomNavigationBarType.shifting,
      items: _navItems,
      currentIndex: _currentIndex,
      selectedItemColor: LayoutBlueColor1,
      unselectedItemColor: NavigationUnselectedItemColor,
      onTap: onNavItemTap,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<AuthService>(
      builder: (BuildContext context, authModel, child) {
        return Scaffold(
            appBar: AppBar(
                elevation: 25,
                shape: const RoundedRectangleBorder(
                    borderRadius: const BorderRadius.vertical(
                        bottom: const Radius.circular(4))),
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
                      imageUrl: authModel.currentUser.photoURL,
                    ),
                  ),
                  SizedBox(
                    width: 10,
                  ),
                ],
                titleSpacing: 0,
                title: Text("Houseflow"),
                leading: Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: const CircleAvatar(
                    backgroundColor: Colors.transparent,
                    backgroundImage: AssetImage(LOGO_DIR_192),
                  ),
                )),
            bottomNavigationBar: navigation(context),
            body: _navPages.elementAt(_currentIndex));
      },
    );
  }
}
