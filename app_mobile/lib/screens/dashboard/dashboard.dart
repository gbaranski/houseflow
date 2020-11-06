import 'dart:async';

import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/screens/dashboard/device_history.dart';
import 'package:houseflow/screens/my_profile/my_profile.dart';
import 'package:houseflow/screens/support/help_screen.dart';
import 'package:houseflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/shared/profile_image.dart';
import 'package:houseflow/widgets/devices/device.dart';
import 'package:infinite_scroll_pagination/infinite_scroll_pagination.dart';
import 'package:provider/provider.dart';

class Dashboard extends StatefulWidget {
  @override
  _DashboardState createState() => _DashboardState();
}

class _DashboardState extends State<Dashboard> {
  final PagingController<int, DocumentSnapshot> _pagingController =
      PagingController(firstPageKey: 0, invisibleItemsThreshold: 1);

  Future<void> onRefresh() async {
    HapticFeedback.vibrate();
    print("Refreshing");

    Completer completer = Completer();

    void Function(PagingStatus) listener;
    listener = (PagingStatus pagingStatus) {
      print(pagingStatus);
      if (pagingStatus == PagingStatus.ongoing ||
          pagingStatus == PagingStatus.noItemsFound) {
        _pagingController.removeStatusListener(listener);
        completer.complete();
      }
    };
    _pagingController.addStatusListener(listener);
    _pagingController.refresh();

    return completer.future;
  }

  @override
  Widget build(BuildContext context) {
    final AuthService authService =
        Provider.of<AuthService>(context, listen: false);
    final hasAnyDevices = authService.firebaseUser.devices.length > 1;

    return RefreshIndicator(
      onRefresh: hasAnyDevices ? onRefresh : () async {},
      color: Colors.blue,
      backgroundColor: Colors.black54,
      child: CustomScrollView(
          physics:
              AlwaysScrollableScrollPhysics().applyTo(BouncingScrollPhysics()),
          slivers: [
            SliverAppBar(
              backgroundColor: Colors.white,
              elevation: 0,
              expandedHeight: 80,
              actions: [
                Padding(
                  padding: const EdgeInsets.only(top: 20),
                  child: GestureDetector(
                    onTap: () => Navigator.push(
                        context,
                        MaterialPageRoute(
                            settings: const RouteSettings(name: 'My profile'),
                            builder: (context) => MyProfile(
                                firebaseUser: authService.firebaseUser,
                                currentUser: authService.currentUser,
                                signOut: authService.signOut))),
                    child: ProfileImage(
                      size: 38,
                      imageUrl: authService.currentUser.photoURL,
                    ),
                  ),
                ),
                SizedBox(
                  width: 10,
                ),
              ],
              title: Padding(
                padding: const EdgeInsets.only(top: 20),
                child: Text(
                  "Hi, ${authService.firebaseUser.username.split(' ')[0]}!",
                  style: TextStyle(
                      color: Colors.black.withAlpha(160),
                      fontSize: 20,
                      fontWeight: FontWeight.w600),
                ),
              ),
            ),
            if (hasAnyDevices) ...[
              SliverGrid(
                gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
                    childAspectRatio: 1.2, maxCrossAxisExtent: 250),
                delegate: SliverChildListDelegate(authService
                    .firebaseUser.devices
                    .map((firebaseDevice) =>
                        Device(context: context, uid: firebaseDevice.uid))
                    .toList()),
              ),
              DeviceHistoryList(
                getDeviceHistory: ([DocumentSnapshot documentSnapshot]) {
                  return FirebaseService.getFirebaseDeviceHistory(
                      authService.firebaseUser.devices, documentSnapshot);
                },
                pagingController: _pagingController,
              )
            ] else
              (SliverToBoxAdapter(
                  child: Container(
                child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        Icons.warning,
                        color: Colors.red,
                        size: 72,
                      ),
                      Text(
                        "Nothing found!",
                        style: TextStyle(fontSize: 24),
                      ),
                      Text(
                        "Sorry, we couldn't any device that belongs to you.",
                        style: TextStyle(fontSize: 13, color: Colors.black45),
                      ),
                      GestureDetector(
                        onTap: () => Navigator.push(
                            context,
                            MaterialPageRoute(
                                settings: const RouteSettings(name: 'Support'),
                                builder: (context) => HelpScreen())),
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Text(
                              "If you need help ",
                              style: TextStyle(
                                  fontSize: 13, color: Colors.black45),
                            ),
                            Text(
                              "contact us",
                              style: TextStyle(
                                  fontSize: 13, color: LayoutBlueColor1),
                            ),
                          ],
                        ),
                      )
                    ]),
              )))
          ]),
    );
  }
}
