import 'dart:convert';

import 'package:firebase_auth/firebase_auth.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/index.dart';
import 'package:houseflow/services/device.dart';
import 'package:houseflow/services/misc.dart';
import 'package:houseflow/services/preferences.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:quick_actions/quick_actions.dart';

class DeviceShortcut {
  final String deviceUID;
  final DeviceAction action;
  final String title;
  final String icon;

  Map<String, dynamic> toMap() {
    return {
      'deviceUID': deviceUID,
      'title': title,
      'icon': icon,
      'action': {
        'name': action.name.stringify(),
        'id': action.id,
      }
    };
  }

  factory DeviceShortcut.fromMap(Map<String, dynamic> map) {
    return DeviceShortcut(
        deviceUID: map['deviceUID'],
        title: map['title'],
        icon: map['icon'],
        action: DeviceAction(
          name: DeviceActionTypes.values.firstWhere(
              (actionType) => actionType.stringify() == map['action']['name']),
          id: map['action']['id'],
        ));
  }

  DeviceShortcut(
      {@required this.deviceUID,
      @required this.title,
      @required this.icon,
      @required this.action});
}

abstract class DeviceActions {
  static String removeActionPrefix(String sentence) {
    return sentence.replaceFirst(new RegExp('action_'), '');
  }

  static Future _actionsCallback(String shortcut, User currentUser) async {
    print("Recieved action from shortcut $shortcut");
    return;

    final DeviceActionTypes deviceRequestActions = DeviceActionTypes.values
        .firstWhere(
            (action) => action.stringify() == removeActionPrefix(shortcut));
    if (deviceRequestActions == null) {
      throw new Exception('Unexpected shortcut $shortcut');
    }

    final geoPoint = await getCurrentGeoPoint();

    final DeviceRequest deviceRequest = DeviceRequest(
        device: DeviceRequestDevice(
            action:
                // temporary solution, but currently every device accepts id: 1
                DeviceAction(name: deviceRequestActions, id: 1)),
        user: DeviceRequestUser(
          geoPoint: geoPoint,
          token: await currentUser.getIdToken(),
        ));

    await sendDeviceRequest(deviceRequest);
  }

  static Future<void> setShortcutItemsFromPreferences(
          AppPreferences preferences) =>
      QuickActions().setShortcutItems(preferences.shortcuts
          .map((shortcut) => ShortcutItem(
                type: jsonEncode({
                  'name': shortcut.action.name.stringify(),
                  'id': shortcut.action.id,
                  'deviceUID': shortcut.deviceUID,
                }),
                localizedTitle: shortcut.title,
                icon: shortcut.icon,
              ))
          .toList());

  static Future<void> initialize(User currentUser) async {
    final QuickActions quickActions = QuickActions();
    quickActions
        .initialize((shortcut) => _actionsCallback(shortcut, currentUser));
    final appPreferences = await AppPreferences.getPreferences();
    setShortcutItemsFromPreferences(appPreferences);
  }
}
