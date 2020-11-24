import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:houseflow/services/device_actions.dart';
import 'package:shared_preferences/shared_preferences.dart';

class AppPreferences {
  List<DeviceShortcut> shortcuts;

  static Future<void> setPreferences(AppPreferences preferences) async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();

    final List<String> jsonEncodedShortcuts = preferences.shortcuts
        .map((shortcut) => jsonEncode(shortcut.toMap()))
        .toList();

    DeviceActions.setShortcutItemsFromPreferences(preferences);
    prefs.setStringList('shortcuts', jsonEncodedShortcuts);
  }

  static Future<AppPreferences> getPreferences() async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    if (!prefs.containsKey('shortcuts')) return AppPreferences(shortcuts: []);
    final List<String> jsonEncodedShortcuts = prefs.getStringList('shortcuts');
    return AppPreferences(
        shortcuts: jsonEncodedShortcuts
            .map((encodedShortcut) =>
                DeviceShortcut.fromMap(jsonDecode(encodedShortcut)))
            .toList());
  }

  AppPreferences({@required this.shortcuts});
}

class DevicePreferences {
  bool notifications;

  Map<String, dynamic> toMap() {
    return {
      'notifications': notifications,
    };
  }

  static Future<void> setPreferences(
      String deviceUID, DevicePreferences preferences) async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    prefs.setBool('$deviceUID/notifications', preferences.notifications);
  }

  static Future<DevicePreferences> getPreferences(String deviceUID) async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    if (!prefs.containsKey('$deviceUID/notifications'))
      return DevicePreferences(notifications: false);
    return DevicePreferences(
        notifications: prefs.getBool('$deviceUID/notifications'));
  }

  DevicePreferences({@required this.notifications});
}
