import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/utils/misc.dart';

class SingleDevice extends StatelessWidget {
  final FirebaseDevice firebaseDevice;

  SingleDevice({@required this.firebaseDevice});

  void unsubscribe(BuildContext context) {
    final SnackBar snackBar =
        SnackBar(content: Text("Unsubscribed from ${firebaseDevice.uid}"));
    FirebaseService.unsubscribeTopic(firebaseDevice.uid).then((_) {
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  void subscribe(BuildContext context) {
    final SnackBar snackBar =
        SnackBar(content: Text("Subscribed to ${firebaseDevice.uid}"));
    FirebaseService.subscribeTopic(firebaseDevice.uid).then((_) {
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  void copyUuid(BuildContext context) {
    Clipboard.setData(ClipboardData(text: firebaseDevice.uid)).then((_) {
      HapticFeedback.vibrate();
      final snackBar = SnackBar(
        content: Text("${firebaseDevice.uid} copied to clipboard"),
      );
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      appBar: PreferredSize(
        preferredSize: Size.fromHeight(80),
        child: AppBar(
          backgroundColor: Colors.white,
          elevation: 0,
          flexibleSpace: Container(
            margin: const EdgeInsets.only(left: 15),
            alignment: Alignment.bottomLeft,
            child: IconButton(
              onPressed: () => Navigator.pop(context),
              tooltip: "Back",
              icon: Icon(
                Icons.arrow_back_ios,
                color: ACTION_ICON_COLOR,
              ),
            ),
          ),
        ),
      ),
      body: Builder(builder: (context) {
        return Container(
          margin: const EdgeInsets.only(left: 20, top: 40),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              GestureDetector(
                onLongPress: () => copyUuid(context),
                child: Text(
                  upperFirstCharacter(firebaseDevice.type),
                  style: TextStyle(fontSize: 36, fontWeight: FontWeight.w600),
                ),
              ),
              Row(
                children: [
                  Text(
                    firebaseDevice.status
                        ? "Connection with device looks great!"
                        : "Connection with device failed",
                    style: TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                        color: ACTION_ICON_COLOR),
                  ),
                  SizedBox(
                    width: 5,
                  ),
                  firebaseDevice.status
                      ? Icon(Icons.check_circle, color: Colors.green)
                      : Icon(Icons.error, color: Colors.orangeAccent),
                ],
              ),
              SizedBox(
                height: 20,
              ),
              ExpansionTile(
                leading: Icon(Icons.sync),
                title: Text("Notifications"),
                children: [
                  ButtonBar(
                      alignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        FlatButton.icon(
                            onPressed: () => subscribe(context),
                            icon: Icon(Icons.notifications),
                            textColor: Color(0xFF3d5a80),
                            label: Text("Subscribe")),
                        FlatButton.icon(
                            onPressed: () => unsubscribe(context),
                            icon: Icon(Icons.notifications_off),
                            textColor: Color(0xFF293241),
                            label: Text("Unsubscribe")),
                      ])
                ],
              )
            ],
          ),
        );
      }),
    );
  }
}
