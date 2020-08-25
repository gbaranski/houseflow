import 'package:app_mobile/models/user.dart';
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:flutter/cupertino.dart';

class AuthService extends ChangeNotifier {
  final auth.FirebaseAuth _auth = auth.FirebaseAuth.instance;
  final FirebaseFirestore _firestore = FirebaseFirestore.instance;

  bool isInitialized = false;

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
    _auth.authStateChanges().listen((event) {
      isInitialized = true;
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
      return await _auth.signOut();
    } catch (e) {
      print(e.toString());
      return null;
    }
  }
}
