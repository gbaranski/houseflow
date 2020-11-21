import 'dart:async';
import 'package:firebase_crashlytics/firebase_crashlytics.dart';
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
  static final FirebaseMessaging _fcm = FirebaseMessaging.instance;
  static final FirebaseAnalytics analytics = FirebaseAnalytics();
  static final FirebaseCrashlytics crashlytics = FirebaseCrashlytics.instance;
  static FirebaseAnalyticsObserver observer =
      FirebaseAnalyticsObserver(analytics: analytics);

  static final FirebaseFunctions functions =
      FirebaseFunctions.instanceFor(region: 'europe-west1');

  static final _usersCollection = _firestore.collection('users');
  static final _devicesCollection = _firestore.collection('devices');

  static void initFcm(BuildContext context) async {
    print("Initializing FCM");
    FirebaseMessaging.onMessage.listen((remoteMessage) => {
          showDialog(
              context: context,
              builder: (context) => AlertDialog(
                    content: ListTile(
                      title: Text(remoteMessage.notification.title),
                      subtitle: Text(remoteMessage.notification.body),
                    ),
                    actions: [
                      FlatButton(
                        child: Text("OK"),
                        onPressed: () => Navigator.of(context).pop(),
                      )
                    ],
                  ))
        });
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

  static Future<QuerySnapshot> getFirebaseDeviceHistory(
      List<FirebaseUserDevice> firebaseDevices,
      [DocumentSnapshot lastVisibleDocument]) async {
    Query query;
    if (lastVisibleDocument != null)
      query = _firestore
          .collectionGroup('history')
          .where('destination.deviceUid',
              whereIn: firebaseDevices.map((device) => device.uid).toList())
          .orderBy('timestamp', descending: true)
          .startAfterDocument(lastVisibleDocument)
          .limit(5);
    else
      query = _firestore
          .collectionGroup('history')
          .where('destination.deviceUid',
              whereIn: firebaseDevices.map((device) => device.uid).toList())
          .orderBy('timestamp', descending: true)
          .limit(5);

    return query.get();
  }

  static Stream<DocumentSnapshot> getFirebaseDeviceSnapshot(String uid) {
    return _devicesCollection.doc(uid).snapshots();
  }

  static Future<void> unsubscribeTopic(String topic) {
    return _fcm.unsubscribeFromTopic(topic);
  }

  static Future<void> subscribeTopic(String topic) {
    _fcm.requestPermission();
    return _fcm.subscribeToTopic(topic);
  }

  static HttpsCallable initializeNewUser() {
    return functions.httpsCallable('initializeNewUser');
  }
}
