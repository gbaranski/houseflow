import 'dart:convert';

import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/index.dart';
import 'package:houseflow/services/notifications.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:http/http.dart' as http;

Future<http.Response> sendDeviceRequest(DeviceRequest deviceRequest) async {
  final response = await http.post('$DEVICE_API_URL/request',
      body: jsonEncode(deviceRequest.toMap()),
      headers: {
        'Content-Type': 'application/json; charset=UTF-8',
      });

  if (response.statusCode == 200 &&
      deviceRequest.device.action.name == DeviceActionTypes.MixWater) {
    Notifications.scheduleNotification(
        title: "Water have been mixed!",
        body: "Water should be warm now, it's time to go",
        duration: const Duration(minutes: 6));
  }

  return response;
}
