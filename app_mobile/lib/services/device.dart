import 'dart:convert';

import 'package:houseflow/models/device.dart';
import 'package:houseflow/shared/constants.dart';
import 'package:http/http.dart' as http;

Future<http.Response> sendDeviceRequest(DeviceRequest deviceRequest) async {
  final response = await http.post('$DEVICE_API_URL/request',
      body: jsonEncode(deviceRequest.toMap()),
      headers: {
        'Content-Type': 'application/json; charset=UTF-8',
      });

  return response;
}
