import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:timezone/data/latest.dart' as tz;
import 'package:timezone/timezone.dart' as tz;

const DEVICE_NOTIFICATION_RESPONSE_CHANNEL_ID = "device_notification_response";
const DEVICE_NOTIFICATION_RESPONSE_CHANNEL_NAME = "Response from devices";
const DEVICE_NOTIFICATION_RESPONSE_CHANNEL_DESCRIPTION =
    "Response from requesting devices action";

const DEVICE_NOTIFICATION_SCHEDULE_CHANNEL_ID = "device_schedule_notification";
const DEVICE_NOTIFICATION_SCHEDULE_CHANNEL_NAME = "Scheduled notification";
const DEVICE_NOTIFICATION_SCHEDULE_CHANNEL_DESCRIPTION =
    "This is scheduled notification from device";

class Notifications {
  static final FlutterLocalNotificationsPlugin
      _flutterLocalNotificationsPlugin = FlutterLocalNotificationsPlugin();

  static Future _onDidReceiveLocalNotification(int id, String title,
      String body, String payload, BuildContext context) async {
    // display a dialog with the notification details, tap ok to go to another page
    showDialog(
      context: context,
      builder: (BuildContext context) => CupertinoAlertDialog(
        title: Text(title),
        content: Text(body),
        actions: [
          CupertinoDialogAction(
              isDefaultAction: true,
              child: Text('Ok'),
              onPressed: () => Navigator.of(context, rootNavigator: true).pop())
        ],
      ),
    );
  }

  static Future _selectNotification(String payload) async {
    if (payload != null) {
      debugPrint('notification payload: $payload');
    }
  }

  static Future init(BuildContext context) async {
    const AndroidInitializationSettings initializationSettingsAndroid =
        AndroidInitializationSettings('app_icon');
    final IOSInitializationSettings initializationSettingsIOS =
        IOSInitializationSettings(
            onDidReceiveLocalNotification: (id, title, body, payload) =>
                _onDidReceiveLocalNotification(
                    id, title, body, payload, context));
    final MacOSInitializationSettings initializationSettingsMacOS =
        MacOSInitializationSettings();
    final InitializationSettings initializationSettings =
        InitializationSettings(
            android: initializationSettingsAndroid,
            iOS: initializationSettingsIOS,
            macOS: initializationSettingsMacOS);

    await _flutterLocalNotificationsPlugin.initialize(initializationSettings,
        onSelectNotification: _selectNotification);
  }

  static Future scheduleNotification(
      {@required String title,
      @required String body,
      @required Duration duration}) async {
    await _flutterLocalNotificationsPlugin.zonedSchedule(
        0,
        title,
        body,
        tz.TZDateTime.now(tz.local).add(duration),
        NotificationDetails(
            android: const AndroidNotificationDetails(
          DEVICE_NOTIFICATION_SCHEDULE_CHANNEL_ID,
          DEVICE_NOTIFICATION_SCHEDULE_CHANNEL_NAME,
          DEVICE_NOTIFICATION_SCHEDULE_CHANNEL_DESCRIPTION,
          importance: Importance.max,
        )),
        androidAllowWhileIdle: true,
        uiLocalNotificationDateInterpretation:
            UILocalNotificationDateInterpretation.absoluteTime);
  }

  static Future showActionResponse(int statusCode) async {
    const AndroidNotificationDetails androidPlatformChannelSpecifics =
        AndroidNotificationDetails(
            DEVICE_NOTIFICATION_RESPONSE_CHANNEL_ID,
            DEVICE_NOTIFICATION_RESPONSE_CHANNEL_NAME,
            DEVICE_NOTIFICATION_RESPONSE_CHANNEL_DESCRIPTION,
            importance: Importance.max,
            priority: Priority.high,
            showWhen: false);
    const NotificationDetails platformChannelSpecifics =
        NotificationDetails(android: androidPlatformChannelSpecifics);
    final bool success = statusCode == 200;
    await _flutterLocalNotificationsPlugin.show(
        0,
        success
            ? "Sending request completed successfully"
            : "Failed, unexpected response $statusCode",
        "Tap to open Houseflow app",
        platformChannelSpecifics);
  }
}
