import 'package:cloud_firestore/cloud_firestore.dart' as firestore;
import 'package:flutter/material.dart';
import 'package:houseflow/models/devices/index.dart';

class FirebaseDevice {
  String ip;
  bool status;
  String type;
  String uid;
  List<DeviceAction> actions;

  factory FirebaseDevice.fromMap(Map<String, dynamic> map) {
    return FirebaseDevice(
        ip: map['ip'],
        status: map['status'],
        type: map['type'],
        uid: map['uid'],
        actions: (map['actions'] as List<dynamic>)
            .map((action) => DeviceAction(
                name: DeviceActionTypes.values.firstWhere((actionType) =>
                    actionType.stringify() == action['name'] as String),
                id: action['id']))
            .toList());
  }
  Map<String, dynamic> toJson() {
    return {
      'ip': ip,
      'status': status,
      'type': type,
      'uid': uid,
      'actions': actions.map((action) => {
            'name': action.name,
            'id': action.id,
          })
    };
  }

  FirebaseDevice(
      {@required this.ip,
      @required this.status,
      @required this.type,
      @required this.uid,
      @required this.actions});
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

class DeviceAction {
  final DeviceActionTypes name;
  final int id;

  DeviceAction({@required this.name, this.id = 1});
}

class DeviceRequestDevice {
  final String uid;
  final DeviceAction action;
  final String data;

  @override
  String toString() {
    return "$uid/action-${action.name}";
  }

  DeviceRequestDevice({@required this.action, this.data, this.uid});
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
        'action': {
          'name': device.action.name.stringify(),
          'id': device.action.id,
        },
        if (device.uid != null) 'uid': device.uid,
        if (device.data != null) 'data': device.data,
      }
    };
  }

  DeviceRequest({@required this.user, @required this.device});
}
