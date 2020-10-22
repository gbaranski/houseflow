import 'dart:async';
import 'package:flutter/material.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:cloud_functions/cloud_functions.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:houseflow/models/user.dart';
import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:firebase_analytics/observer.dart';

class FirebaseService {
  static final FirebaseFirestore _firestore = FirebaseFirestore.instance;
  static final FirebaseMessaging _fcm = FirebaseMessaging();
  static final FirebaseAnalytics analytics = FirebaseAnalytics();
  static FirebaseAnalyticsObserver observer =
      FirebaseAnalyticsObserver(analytics: analytics);

  static final CloudFunctions functions =
      CloudFunctions(region: 'europe-west1');

  static final _usersCollection = _firestore.collection('users');
  static final _devicesCollection = _firestore.collection('devices');

  static void initFcm(BuildContext context) async {
    print("Initializing FCM");
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

  static Stream<DocumentSnapshot> firebaseUserStream(auth.User user) {
    return _usersCollection.doc(user.uid).snapshots();
  }

  static Future<FirebaseUser> convertToFirebaseUser(auth.User user) async {
    if (user == null) return null;
    if (user.isAnonymous)
      return FirebaseUser(
          devices: [],
          role: 'user',
          uid: user.uid,
          isAnonymous: true,
          username: "Anonymous");
    final doc = await _usersCollection.doc(user.uid).get();
    if (!doc.exists) {
      print("Does not exist");
      return null;
    }
    final data = doc.data();

    print("Firebase data about user: $data");
    return FirebaseUser.fromMap(data);
  }

  static List<Stream<QuerySnapshot>> firebaseDevicesHistoryStream(
      List<FirebaseUserDevice> firebaseDevices) {
    return firebaseDevices
        .map((device) => _devicesCollection
            .doc(device.uid)
            .collection('history')
            .snapshots())
        .toList();
  }

  static Stream<DocumentSnapshot> getFirebaseDeviceSnapshot(String uid) {
    return _devicesCollection.doc(uid).snapshots();
  }

  static updateFirebaseDeviceData(String uid, Map<String, dynamic> deviceData) {
    _devicesCollection.doc(uid).update({"data": deviceData});
  }

  static Future<void> unsubscribeTopic(String topic) {
    return _fcm.unsubscribeFromTopic(topic);
  }

  static Future<void> subscribeTopic(String topic) {
    return _fcm.subscribeToTopic(topic);
  }

  static HttpsCallable initializeNewUser() {
    return functions.getHttpsCallable(functionName: 'initializeNewUser');
  }
}
