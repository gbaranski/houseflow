import 'package:homeflow/services/firebase.dart';
import 'package:flutter/material.dart';
import 'package:homeflow/models/user.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:google_sign_in/google_sign_in.dart';

class AuthService extends ChangeNotifier {
  final auth.FirebaseAuth _auth = auth.FirebaseAuth.instance;
  final List<String> _subscribedTopics = [];
  auth.User currentUser;
  FirebaseUser firebaseUser;

  AuthStatus authStatus = AuthStatus.NOT_DETERMINED;

  // auth change user stream
  Stream<auth.User> get user {
    _auth.authStateChanges().listen((event) async {
      if (event == null) {
        authStatus = AuthStatus.NOT_LOGGED_IN;
      } else {
        firebaseUser = await FirebaseService.convertToFirebaseUser(event);
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
      return await FirebaseService.convertToFirebaseUser(user);
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
      _subscribedTopics
          .forEach((topic) => FirebaseService.unsubscribeTopic(topic));
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

  void subscribeToAllDevicesTopic(List<FirebaseUserDevice> userDevices) {
    userDevices
        .forEach((element) => FirebaseService.subscribeTopic(element.uid));
  }
}
