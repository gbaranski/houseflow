import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:houseflow/screens/additional_view/index.dart';
import 'package:shared_preferences/shared_preferences.dart';

class DeviceSettings {
  bool notifications;
  bool shortcut;

  Map<String, dynamic> toMap() {
    return {
      'notifications': notifications,
      'shortcut': shortcut,
    };
  }

  factory DeviceSettings.fromMap(Map<String, dynamic> map) {
    return DeviceSettings(
        notifications: map['notifications'], shortcut: map['shortcut']);
  }

  DeviceSettings({@required this.notifications, @required this.shortcut});
}

class SingleDevice extends StatefulWidget {
  final FirebaseDevice firebaseDevice;

  SingleDevice({@required this.firebaseDevice});

  @override
  _SingleDeviceState createState() => _SingleDeviceState();
}

class _SingleDeviceState extends State<SingleDevice> {
  DeviceSettings _deviceSettings;

  Future<DeviceSettings> loadDeviceSettings() async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    if (!prefs.containsKey('${widget.firebaseDevice.uid}/settings'))
      return null;

    final encodedPreferences =
        prefs.getString('${widget.firebaseDevice.uid}/settings');
    return DeviceSettings.fromMap(jsonDecode(encodedPreferences));
  }

  Future<void> saveDeviceSettings() async {
    if (_deviceSettings == null) return;
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    final encodedPreferences = jsonEncode(_deviceSettings.toMap());
    await prefs.setString(
        '${widget.firebaseDevice.uid}/settings', encodedPreferences);

    if (_deviceSettings.notifications == true)
      FirebaseService.subscribeTopic(widget.firebaseDevice.uid);
    else
      FirebaseService.unsubscribeTopic(widget.firebaseDevice.uid);
  }

  void unsubscribe(BuildContext context) {
    final SnackBar snackBar = SnackBar(
        content: Text("Unsubscribed from ${widget.firebaseDevice.uid}"));
    FirebaseService.unsubscribeTopic(widget.firebaseDevice.uid).then((_) {
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  void subscribe(BuildContext context) {
    final SnackBar snackBar =
        SnackBar(content: Text("Subscribed to ${widget.firebaseDevice.uid}"));
    FirebaseService.subscribeTopic(widget.firebaseDevice.uid).then((_) {
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  void copyUuid(BuildContext context) {
    Clipboard.setData(ClipboardData(text: widget.firebaseDevice.uid)).then((_) {
      HapticFeedback.vibrate();
      final snackBar = SnackBar(
        content: Text("${widget.firebaseDevice.uid} copied to clipboard"),
      );
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  @override
  void initState() {
    super.initState();
    loadDeviceSettings().then((loadedDeviceSettings) {
      if (!mounted) return;
      setState(() {
        _deviceSettings = loadedDeviceSettings != null
            ? loadedDeviceSettings
            : DeviceSettings(notifications: false, shortcut: false);
      });
    }).catchError((e) => print("Error when reading device settings $e"));
  }

  @override
  Widget build(BuildContext context) {
    return AdditionalView(
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () => saveDeviceSettings(),
        icon: Icon(Icons.save),
        backgroundColor: Colors.deepPurple,
        label: Text("Save"),
      ),
      body: Builder(builder: (context) {
        return Container(
          margin: const EdgeInsets.only(left: 20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              GestureDetector(
                onLongPress: () => copyUuid(context),
                child: Text(
                  upperFirstCharacter(widget.firebaseDevice.type),
                  style: TextStyle(fontSize: 36, fontWeight: FontWeight.w600),
                ),
              ),
              Row(
                children: [
                  Text(
                    widget.firebaseDevice.status
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
                  widget.firebaseDevice.status
                      ? Icon(Icons.check_circle, color: Colors.green)
                      : Icon(Icons.error, color: Colors.orangeAccent),
                ],
              ),
              SizedBox(
                height: 20,
              ),
              if (_deviceSettings != null) ...[
                CheckboxListTile(
                  title: Text("Notifications"),
                  value: _deviceSettings.notifications,
                  onChanged: (state) {
                    setState(() {
                      _deviceSettings.notifications = state;
                    });
                  },
                ),
                CheckboxListTile(
                  title: Text("Action shortcut"),
                  value: _deviceSettings.shortcut,
                  onChanged: (state) {
                    setState(() {
                      _deviceSettings.shortcut = state;
                    });
                  },
                ),
              ]
            ],
          ),
        );
      }),
    );
  }
}
