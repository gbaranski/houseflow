import 'dart:async';

import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/screens/my_profile/my_profile.dart';
import 'package:houseflow/screens/support/help_screen.dart';
import 'package:houseflow/services/auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/shared/profile_image.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:houseflow/widgets/device_single_history.dart';
import 'package:houseflow/widgets/devices/relayCard.dart';
import 'package:infinite_scroll_pagination/infinite_scroll_pagination.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

class Dashboard extends StatefulWidget {
  @override
  _DashboardState createState() => _DashboardState();
}

const tenMinutesInMillis = 1000 * 10 * 60;

class _DashboardState extends State<Dashboard> {
  Widget device(BuildContext context, String uid) {
    return StreamBuilder(
      stream: FirebaseService.getFirebaseDeviceSnapshot(uid),
      builder:
          (BuildContext context, AsyncSnapshot<DocumentSnapshot> snapshot) {
        if (snapshot.hasError) return Text("Error occured");
        if (snapshot.connectionState == ConnectionState.waiting)
          return Container();
        final Map<String, dynamic> data = snapshot.data.data();
        final FirebaseDevice firebaseDevice = FirebaseDevice.fromMap(data);
        // if (!firebaseDevice.status) return InactiveDevice(firebaseDevice);

        final iconData = getDeviceIcon(firebaseDevice.type);
        switch (firebaseDevice.type) {
          case 'WATERMIXER':
            return RelayCard(
              cardColor: Color.fromRGBO(79, 119, 149, 1),
              firebaseDevice: firebaseDevice,
              iconData: iconData,
              getNewDeviceData: () =>
                  DateTime.now().millisecondsSinceEpoch + tenMinutesInMillis,
            );
          case 'GATE':
            return RelayCard(
              cardColor: Color.fromRGBO(103, 151, 109, 1),
              firebaseDevice: firebaseDevice,
              iconData: iconData,
              getNewDeviceData: () => DateTime.now().millisecondsSinceEpoch,
            );
          case 'GARAGE':
            return RelayCard(
              cardColor: Color.fromRGBO(183, 111, 110, 1),
              firebaseDevice: firebaseDevice,
              iconData: iconData,
              getNewDeviceData: () => DateTime.now().millisecondsSinceEpoch,
            );

          default:
            {
              return const Text("Some error occured");
            }
        }
      },
    );
  }

  final PagingController<int, DocumentSnapshot> _pagingController =
      PagingController(firstPageKey: 0, invisibleItemsThreshold: 1);

  Future<void> updateDeviceHistory(int pageKey) async {
    try {
      final lastDocument = _pagingController.itemList == null
          ? null
          : _pagingController.itemList[pageKey - 1];

      print(
          "Fetching device history i: $pageKey, last visible doc ${lastDocument?.id}");
      QuerySnapshot snapshot;
      if (lastDocument != null) {
        snapshot = await FirebaseService.getFirebaseDeviceHistory(
            AuthService.firebaseUser.devices, lastDocument);
      } else
        snapshot = await FirebaseService.getFirebaseDeviceHistory(
            AuthService.firebaseUser.devices);

      _pagingController.appendPage(
          snapshot.docs, pageKey + snapshot.docs.length);
    } catch (e) {
      print("Error occured when fetching device history $e");
      _pagingController.error = e;
    }
  }

  @override
  void initState() {
    super.initState();
    _pagingController.addPageRequestListener((pageKey) {
      updateDeviceHistory(pageKey);
    });
  }

  Future<void> onRefresh() async {
    HapticFeedback.vibrate();
    print("Refreshing");

    Completer completer = Completer();

    void Function(PagingStatus) listener;
    listener = (PagingStatus pagingStatus) {
      print(pagingStatus);
      if (pagingStatus == PagingStatus.ongoing) {
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
    final hasAnyDevices = AuthService.firebaseUser.devices.length > 1;
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
                                firebaseUser: AuthService.firebaseUser,
                                currentUser: AuthService.currentUser,
                                signOut: AuthService.signOut))),
                    child: ProfileImage(
                      size: 38,
                      imageUrl: AuthService.currentUser.photoURL,
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
                  "Hi, ${AuthService.firebaseUser.username.split(' ')[0]}!",
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
                delegate: SliverChildListDelegate(AuthService
                    .firebaseUser.devices
                    .map(
                        (firebaseDevice) => device(context, firebaseDevice.uid))
                    .toList()),
              ),
              PagedSliverList<int, DocumentSnapshot>(
                key: Key('deviceHistoryList'),
                pagingController: _pagingController,
                builderDelegate: PagedChildBuilderDelegate<DocumentSnapshot>(
                    itemBuilder: (context, item, index) => SingleDeviceHistory(
                          deviceRequest:
                              DeviceHistory.fromJson(item.data(), item.id),
                        )),
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

  @override
  void dispose() {
    _pagingController.dispose();
    super.dispose();
  }
}
