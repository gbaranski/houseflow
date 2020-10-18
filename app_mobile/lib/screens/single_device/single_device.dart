import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/utils/misc.dart';

class SingleDevice extends StatelessWidget {
  final FirebaseDevice firebaseDevice;

  SingleDevice({@required this.firebaseDevice});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(upperFirstCharacter(firebaseDevice.type)),
      ),
      body: Builder(builder: (context) {
        final subscribe = () {
          final SnackBar snackBar =
              SnackBar(content: Text("Subscribed to ${firebaseDevice.uid}"));
          FirebaseService.subscribeTopic(firebaseDevice.uid).then((_) {
            Scaffold.of(context).showSnackBar(snackBar);
          });
        };

        final unsubscribe = () {
          final SnackBar snackBar = SnackBar(
              content: Text("Unsubscribed from ${firebaseDevice.uid}"));
          FirebaseService.unsubscribeTopic(firebaseDevice.uid).then((_) {
            Scaffold.of(context).showSnackBar(snackBar);
          });
        };

        return Container(
          margin: const EdgeInsets.all(15),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                  "Connection status: ${firebaseDevice.status ? "connected" : "disconnected"}"),
              Text("Last connected IP Address: ${firebaseDevice.ip}"),
              ButtonBar(
                alignment: MainAxisAlignment.start,
                children: [
                  OutlinedButton(
                    onPressed: () {
                      Clipboard.setData(ClipboardData(text: firebaseDevice.uid))
                          .then((_) {
                        HapticFeedback.vibrate();
                        final snackBar = SnackBar(
                          content:
                              Text("${firebaseDevice.uid} copied to clipboard"),
                        );
                        Scaffold.of(context).showSnackBar(snackBar);
                      });
                    },
                    child: Text("Copy UID to clipboard"),
                  ),
                  OutlinedButton(
                    onPressed: subscribe,
                    child: Text("Subscribe to notifications"),
                  ),
                  OutlinedButton(
                    onPressed: unsubscribe,
                    child: Text("Unsubscribe to notifications"),
                  )
                ],
              ),
            ],
          ),
        );
      }),
    );
  }
}
