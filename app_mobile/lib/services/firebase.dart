import 'package:flutter/material.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:homeflow/models/user.dart';

class FirebaseService {
  static final FirebaseFirestore _firestore = FirebaseFirestore.instance;
  static final FirebaseMessaging _fcm = FirebaseMessaging();

  static final _usersCollection = _firestore.collection('users');
  static final _devicesCollection = _firestore.collection('devices');

  static void initFcm(BuildContext context) async {
    _fcm.configure(
      onMessage: (Map<String, dynamic> message) async {
        print("onMessage: $message");
        showDialog(
          context: context,
          builder: (context) => AlertDialog(
            content: ListTile(
              title: Text(message['notification']['title']),
              subtitle: Text(message['notification']['body']),
            ),
            actions: <Widget>[
              FlatButton(
                child: Text('OK'),
                onPressed: () => Navigator.of(context).pop(),
              ),
            ],
          ),
        );
      },
      onLaunch: (Map<String, dynamic> message) async {
        print("onLaunch: $message");
      },
      onResume: (Map<String, dynamic> message) async {
        print("onResume: $message");
      },
    );
    final String token = await _fcm.getToken();
    print("FCM TOKEN: $token");
  }

  static Future<FirebaseUser> convertToFirebaseUser(auth.User user) async {
    if (user == null) return null;
    if (user.isAnonymous)
      return FirebaseUser(
          devices: [], role: 'user', uid: user.uid, isAnonymous: true);
    final doc = await _usersCollection.doc(user.uid).get();
    if (!doc.exists) return null;
    final data = doc.data();

    final userDevices = (data['devices'] as List<dynamic>)
        .map((device) => FirebaseUserDevice(
            notification: device['notification'], uid: device['uid']))
        .toList();

    print("UserDevices runtimetype: ${userDevices.runtimeType}");
    print("Firebase data about user: $data");
    return FirebaseUser(
      uid: data['uid'],
      role: data['role'],
      devices: userDevices,
      isAnonymous: false,
    );
  }

  static Query getFirebaseDevicesQueries(FirebaseUser firebaseUser) {
    print("FirebaseUser devices: ${firebaseUser.devices}");
    final List<String> uidList =
        firebaseUser.devices.map((device) => device.uid).toList();
    return _devicesCollection
        .where("uid", whereIn: ['ba15f964-0686-42f9-8ae2-dee17a445075']);
  }

  static void unsubscribeTopic(String topic) {
    _fcm.unsubscribeFromTopic(topic);
  }

  static void subscribeTopic(String topic) {
    _fcm.subscribeToTopic(topic);
  }
}
