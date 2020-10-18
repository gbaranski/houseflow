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

  Widget navigation(BuildContext context) {
    const BorderRadius borderRadius = BorderRadius.only(
        topLeft: Radius.circular(30), topRight: Radius.circular(30));
    return Container(
      decoration: const BoxDecoration(borderRadius: borderRadius),
      child: ClipRRect(
        borderRadius: borderRadius,
        child: BottomNavigationBar(
          backgroundColor: Colors.white,
          selectedFontSize: 14,
          type: BottomNavigationBarType.shifting,
          items: _navItems,
          currentIndex: _currentIndex,
          selectedItemColor: LayoutBlueColor1,
          unselectedItemColor: NavigationUnselectedItemColor,
          onTap: onNavItemTap,
        ),
      ),
    );
  }

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
                        imageUrl: authModel.currentUser.photoURL,
                      ),
                    ),
                    SizedBox(
                      width: 10,
                    ),
                  ],
                  title: Text(
                    "Hi, ${authModel.currentUser.displayName.split(' ')[0]}!",
                    style: TextStyle(
                        color: Colors.black.withAlpha(160),
                        fontSize: 20,
                        fontWeight: FontWeight.w600),
                  ),
                ),
              ),
            ),
            bottomNavigationBar: navigation(context),
            body: _navPages.elementAt(_currentIndex));
      },
    );
  }
}
