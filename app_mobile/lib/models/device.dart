import 'package:flutter/material.dart';

class FirebaseDevice {
  Map<String, dynamic> data;
  String ip;
  bool status;
  String type;
  String uid;

  factory FirebaseDevice.fromMap(Map<String, dynamic> map) {
    return FirebaseDevice(
        data: map['data'],
        ip: map['ip'],
        status: map['status'],
        type: map['type'],
        uid: map['uid']);
  }
  Map<String, dynamic> toJson() {
    return {
      'data': data,
      'ip': ip,
      'status': status,
      'type': type,
      'uid': uid,
    };
  }

  FirebaseDevice(
      {@required this.data,
      @required this.ip,
      @required this.status,
      @required this.type,
      @required this.uid});
}

class DeviceHistory {
  final String ipAddress;
  final String request;
  final String username;
  final String deviceUid;
  final String deviceType;
  final int timestamp;
  final String docUid;

  factory DeviceHistory.fromJson(Map<String, dynamic> json, String docUid) =>
      DeviceHistory(
        ipAddress: json['ipAddress'],
        request: json['request'],
        username: json['username'],
        timestamp: json['timestamp'],
        deviceUid: json['deviceUid'],
        deviceType: json['deviceType'],
        docUid: docUid,
      );

  String stringifyRequest() {
    switch (request) {
      case 'relay1':
        switch (deviceType) {
          case 'WATERMIXER':
            return 'Mix water';
          case 'GATE':
            return 'Open the gate';
          case 'GARAGE':
            return 'Open the garage';
          case 'LIGHT':
            return 'Toggle lights';

          default:
            return "Unrecognized action";
        }
        break;
      default:
        return 'Unrecognized action';
    }
  }

  DeviceHistory({
    @required this.ipAddress,
    @required this.request,
    @required this.username,
    @required this.timestamp,
    @required this.deviceUid,
    @required this.deviceType,
    @required this.docUid,
  });
}

class GeoPoint {
  final double latitude;
  final double longitude;
  GeoPoint({@required this.latitude, @required this.longitude});
}

class DeviceRequestUser {
  final String token;
  final GeoPoint geoPoint;
  DeviceRequestUser({@required this.token, @required this.geoPoint});
}

class DeviceRequestDevice {
  final String uid;
  final int gpio;
  final String action;
  final String data;

  @override
  String toString() {
    return "$uid/$action$gpio";
  }

  DeviceRequestDevice(
      {@required this.uid,
      @required this.gpio,
      @required this.action,
      this.data});
}

class DeviceRequest {
  final DeviceRequestUser user;
  final DeviceRequestDevice device;

  Map<String, dynamic> toMap() {
    return {
      'user': {
        'token': user.token,
        'geoPoint': {
          'latitude': user.geoPoint.latitude,
          'longitude': user.geoPoint.longitude,
        }
      },
      'device': {
        'uid': device.uid,
        'gpio': device.gpio,
        'action': device.action,
        if (device.data != null) 'data': device.data,
      }
    };
  }

  DeviceRequest({@required this.user, @required this.device});
}
