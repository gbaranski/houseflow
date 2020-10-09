import 'package:homeflow/screens/dashboard/dashboard.dart';
import 'package:homeflow/screens/my_profile/my_profile.dart';
import 'package:homeflow/screens/settings/settings.dart';
import 'package:homeflow/shared/constants.dart';
import 'package:flutter/material.dart';

class Home extends StatefulWidget {
  @override
  _HomeState createState() => _HomeState();
}

class _HomeState extends State<Home> {
  static final List<Widget> _navPages = <Widget>[
    Dashboard(),
    MyProfile(),
    Settings(),
  ];

  static const List<BottomNavigationBarItem> _navItems = [
    BottomNavigationBarItem(
      icon: Icon(Icons.dashboard),
      title: Text("Dashboard"),
    ),
    BottomNavigationBarItem(
      icon: Icon(Icons.person),
      title: Text("My profile"),
    ),
    BottomNavigationBarItem(
      icon: Icon(Icons.settings),
      title: Text("Settings"),
    ),
  ];

  int _currentIndex = 0;

  void onNavItemTap(int index) {
    setState(() {
      _currentIndex = index;
    });
  }

  BottomNavigationBar navigation(BuildContext context) {
    return BottomNavigationBar(
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
    return Scaffold(
        appBar: AppBar(
          backgroundColor: LayoutBlueColor1,
          title: Text("Homeflow"),
        ),
        bottomNavigationBar: navigation(context),
        body: _navPages.elementAt(_currentIndex));
  }
}
