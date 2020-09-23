import 'package:homeflow/models/device.dart';
import 'package:flutter/material.dart';

class ServerResponse {
  final String requestType;
  final List<ActiveDevice> data;

  ServerResponse({this.requestType, this.data});

  factory ServerResponse.fromJson(Map<String, dynamic> json) {
    final activeDevices = json['data'].map((devicesJson) {
      return new ActiveDevice(
        data: devicesJson['data'],
        ip: devicesJson['ip'],
        type: devicesJson['type'],
        uid: devicesJson['uid'],
      );
    });
    return ServerResponse(
        data: activeDevices.toList().cast<ActiveDevice>(),
        requestType: json['requestType']);
  }
}
