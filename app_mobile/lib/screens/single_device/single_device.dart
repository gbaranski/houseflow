import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/index.dart';
import 'package:houseflow/services/device_actions.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/preferences.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:houseflow/screens/additional_view/index.dart';

class SingleDevice extends StatefulWidget {
  final FirebaseDevice firebaseDevice;

  SingleDevice({@required this.firebaseDevice});

  @override
  _SingleDeviceState createState() => _SingleDeviceState();
}

class _SingleDeviceState extends State<SingleDevice> {
  AppPreferences _appPreferences;
  DevicePreferences _devicePreferences;

  void unsubscribe(BuildContext context) =>
      FirebaseService.unsubscribeTopic(widget.firebaseDevice.uid);
  void subscribe(BuildContext context) =>
      FirebaseService.subscribeTopic(widget.firebaseDevice.uid);

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
    DevicePreferences.getPreferences(widget.firebaseDevice.uid)
        .then((preferences) {
      if (!mounted) return;
      setState(() {
        _devicePreferences = preferences != null
            ? preferences
            : DevicePreferences(notifications: false);
      });
    });
    AppPreferences.getPreferences().then((preferences) {
      if (!mounted) return;
      setState(() {
        _appPreferences =
            preferences != null ? preferences : AppPreferences(shortcuts: []);
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return AdditionalView(
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          DevicePreferences.setPreferences(
              widget.firebaseDevice.uid, _devicePreferences);
          AppPreferences.setPreferences(_appPreferences);
        },
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
              if (_devicePreferences != null && _appPreferences != null) ...[
                CheckboxListTile(
                  title: Text("Notifications"),
                  value: _devicePreferences.notifications,
                  onChanged: (state) {
                    setState(() {
                      _devicePreferences.notifications = state;
                    });
                  },
                ),
                ...widget.firebaseDevice.actions
                    .map((action) => CheckboxListTile(
                          title: Text(
                              "Shortcut for ${action.name.stringify2().toLowerCase()}"),
                          value: _appPreferences.shortcuts.any((shortcut) =>
                              shortcut.deviceUID == widget.firebaseDevice.uid &&
                              shortcut.action.name == action.name &&
                              shortcut.action.id == action.id),
                          onChanged: (state) {
                            setState(() {
                              if (state)
                                _appPreferences.shortcuts.add(DeviceShortcut(
                                    title: action.name.stringify2(),
                                    icon:
                                        'device_${widget.firebaseDevice.type.toLowerCase()}',
                                    action: action,
                                    deviceUID: widget.firebaseDevice.uid));
                              else
                                _appPreferences.shortcuts.removeWhere(
                                    (shortcut) =>
                                        shortcut.deviceUID ==
                                            widget.firebaseDevice.uid &&
                                        shortcut.action.name == action.name);
                            });
                          },
                        ))
              ]
            ],
          ),
        );
      }),
    );
  }
}
