import 'package:homeflow/models/device.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/material.dart';
import 'package:homeflow/models/user.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:google_sign_in/google_sign_in.dart';

class AuthService extends ChangeNotifier {
  final auth.FirebaseAuth _auth = auth.FirebaseAuth.instance;
  final FirebaseFirestore _firestore = FirebaseFirestore.instance;
  final FirebaseMessaging _fcm = FirebaseMessaging();
  final List<String> _subscribedTopics = [];
  auth.User currentUser;
  FirebaseUser firebaseUser;

  AuthStatus authStatus = AuthStatus.NOT_DETERMINED;

  void initFcm(BuildContext context) async {
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
        // TODO optional
      },
      onResume: (Map<String, dynamic> message) async {
        print("onResume: $message");
        // TODO optional
      },
    );
    final String token = await _fcm.getToken();
    print("FCM TOKEN: $token");
  }

  void subscribeToAllDevicesTopic(List<FirebaseDevice> devices) {
    devices.forEach((device) {
      _subscribedTopics.add(device.uid);
      _fcm.subscribeToTopic(device.uid);
    });
  }

  Future<FirebaseUser> _convertToFirebaseUser(auth.User user) async {
    if (user == null) return null;
    if (user.isAnonymous) return FirebaseUser(uid: user.uid, isAnonymous: true);
    final doc = await _firestore.collection('users').doc(user.uid).get();
    if (!doc.exists) return null;
    final data = doc.data();

    return FirebaseUser(
      uid: data['uid'],
      role: data['role'],
      devices: data['devices'],
      isAnonymous: false,
    );
  }

  // auth change user stream
  Stream<auth.User> get user {
    _auth.authStateChanges().listen((event) async {
      if (event == null) {
        authStatus = AuthStatus.NOT_LOGGED_IN;
      } else {
        firebaseUser = await _convertToFirebaseUser(event);
        currentUser = event;
        authStatus = AuthStatus.LOGGED_IN;
      }
      print("Initialized");
      notifyListeners();
    });
    return _auth.authStateChanges();
  }

  Future signInAnon() async {
    try {
      auth.UserCredential result = await _auth.signInAnonymously();
      auth.User user = result.user;
      return await _convertToFirebaseUser(user);
    } catch (e) {
      print(e.toString());
      return null;
    }
  }

  Future<auth.UserCredential> signInWithGoogle() async {
    // Trigger the authentication flow
    final GoogleSignInAccount googleUser = await GoogleSignIn().signIn();

    // Obtain the auth details from the request
    final GoogleSignInAuthentication googleAuth =
        await googleUser.authentication;

    // Create a new credential
    final auth.GoogleAuthCredential credential =
        auth.GoogleAuthProvider.credential(
      accessToken: googleAuth.accessToken,
      idToken: googleAuth.idToken,
    );

    // Once signed in, return the UserCredential
    return await auth.FirebaseAuth.instance.signInWithCredential(credential);
  }

  Future signInWithEmailAndPassword(String email, String password) async {
    auth.UserCredential result = await _auth.signInWithEmailAndPassword(
        email: email, password: password);
    auth.User user = result.user;
    return user;
  }

  Future registerWithEmailAndPassword(String email, String password) async {
    auth.UserCredential result = await _auth.createUserWithEmailAndPassword(
        email: email, password: password);
    auth.User user = result.user;
    return user;
  }

  Future signOut() async {
    try {
      _subscribedTopics.forEach((topic) => _fcm.unsubscribeFromTopic(topic));
      firebaseUser = null;
      currentUser = null;
      return await _auth.signOut();
    } catch (e) {
      print(e.toString());
      return null;
    }
  }

  Future<String> getIdToken() {
    return currentUser.getIdToken(true);
  }
}
