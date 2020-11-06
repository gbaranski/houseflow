import 'package:cloud_firestore/cloud_firestore.dart' as firestore;
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

class DeviceHistorySource {
  final GeoPoint geoPoint;
  final String ipAddress;
  final String userUid;
  final String username;
  DeviceHistorySource(
      {@required this.geoPoint,
      @required this.ipAddress,
      @required this.userUid,
      @required this.username});
}

class DeviceHistoryDestination {
  final String deviceType;
  final String deviceUid;
  DeviceHistoryDestination(
      {@required this.deviceUid, @required this.deviceType});
}

class DeviceHistory {
  final DeviceHistorySource source;
  final DeviceHistoryDestination destination;
  final String action;
  final int timestamp;
  final String type;
  final String docUid;

  factory DeviceHistory.fromMap(Map<String, dynamic> map, String docUid) =>
      DeviceHistory(
        source: DeviceHistorySource(
          geoPoint: GeoPoint.fromFirestoreGeoPoint(map['source']['geoPoint']),
          ipAddress: map['source']['ipAddress'],
          userUid: map['source']['userUid'],
          username: map['source']['username'],
        ),
        destination: DeviceHistoryDestination(
            deviceType: map['destination']['deviceType'],
            deviceUid: map['destination']['deviceUid']),
        action: map['action'],
        type: map['type'],
        timestamp: map['timestamp'],
        docUid: docUid,
      );

  String stringifyRequest() {
    switch (destination.deviceType) {
      case 'WATERMIXER':
        return 'Mix water';
      case 'GATE':
        return 'Open the gate';
      case 'GARAGE':
        return 'Open the garage';
      case 'LIGHT':
        return 'Toggle lights';
      default:
        return 'Unrecognized action';
    }
  }

  DeviceHistory({
    @required this.action,
    @required this.destination,
    @required this.source,
    @required this.timestamp,
    @required this.type,
    @required this.docUid,
  });
}

class GeoPoint {
  final double latitude;
  final double longitude;

  factory GeoPoint.fromFirestoreGeoPoint(firestore.GeoPoint geoPoint) =>
      GeoPoint(latitude: geoPoint.latitude, longitude: geoPoint.longitude);

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
