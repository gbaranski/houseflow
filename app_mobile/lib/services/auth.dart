import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/models/user.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:google_sign_in/google_sign_in.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:sign_in_with_apple/sign_in_with_apple.dart';

class AuthService extends ChangeNotifier {
  static final auth.FirebaseAuth _auth = auth.FirebaseAuth.instance;
  auth.User _currentUser;
  FirebaseUser _firebaseUser;

  AuthStatus authStatus = AuthStatus.NOT_DETERMINED;

  auth.User get currentUser => _currentUser;

  set firebaseUser(FirebaseUser firebaseUser) {
    _firebaseUser = firebaseUser;
    if (firebaseUser == null) throw "Firebase user cannot be null";
    if (_currentUser == null)
      throw "Attempted to update firebaseUser, but currentUser is not defined";
    authStatus = AuthStatus.LOGGED_IN;
  }

  FirebaseUser get firebaseUser => _firebaseUser;

  AuthService() {
    _auth.authStateChanges().listen((event) {
      if (event == null)
        authStatus = AuthStatus.NOT_LOGGED_IN;
      else {
        authStatus = AuthStatus.NOT_RETREIVED_FIRESTORE;
        _currentUser = event;
      }
      notifyListeners();
      print("Auth state changed!");
    });
  }

  Future signInAnon() async {
    try {
      auth.UserCredential result = await _auth.signInAnonymously();
      auth.User user = result.user;
      FirebaseService.analytics.logSignUp(signUpMethod: 'Anonymous');
      return await FirebaseService.convertToFirebaseUser(user);
    } catch (e) {
      print(e.toString());
      return null;
    }
  }

  Future signInWithGoogle() async {
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

    final credentials =
        await auth.FirebaseAuth.instance.signInWithCredential(credential);
    if (credentials.additionalUserInfo.isNewUser)
      FirebaseService.analytics.logSignUp(signUpMethod: 'Google');
    else
      FirebaseService.analytics.logLogin(loginMethod: 'Google');
    // Once signed in, return the UserCredential
    FirebaseService.crashlytics.setUserIdentifier(credentials.user.uid);
    return credentials;
  }

  static Future<auth.OAuthCredential> _createAppleOAuthCred() async {
    final nonce = createNonce(32);
    final nativeAppleCred = Platform.isIOS
        ? await SignInWithApple.getAppleIDCredential(
            scopes: [
              AppleIDAuthorizationScopes.email,
              AppleIDAuthorizationScopes.fullName,
            ],
            nonce: sha256.convert(utf8.encode(nonce)).toString(),
          )
        : await SignInWithApple.getAppleIDCredential(
            scopes: [
              AppleIDAuthorizationScopes.email,
              AppleIDAuthorizationScopes.fullName,
            ],
            webAuthenticationOptions: WebAuthenticationOptions(
              redirectUri: Uri.parse(
                  'https://pool-airy-nurse.glitch.me/callbacks/sign_in_with_apple'),
              clientId: 'com.gbaranski.houseflow.service',
            ),
            nonce: sha256.convert(utf8.encode(nonce)).toString(),
          );

    return new auth.OAuthCredential(
      providerId: "apple.com", // MUST be "apple.com"
      signInMethod: "oauth", // MUST be "oauth"
      accessToken: nativeAppleCred
          .identityToken, // propagate Apple ID token to BOTH accessToken and idToken parameters
      idToken: nativeAppleCred.identityToken,
      rawNonce: nonce,
    );
  }

  Future signInWithApple() async {
    final oauthCred = await _createAppleOAuthCred();
    final credentials = await _auth.signInWithCredential(oauthCred);
    if (credentials.additionalUserInfo.isNewUser)
      FirebaseService.analytics.logSignUp(signUpMethod: 'Apple');
    else
      FirebaseService.analytics.logLogin(loginMethod: 'Apple');

    FirebaseService.crashlytics.setUserIdentifier(credentials.user.uid);
    return credentials;
  }

  Future signInWithEmailAndPassword(String email, String password) async {
    auth.UserCredential result = await _auth.signInWithEmailAndPassword(
        email: email, password: password);
    auth.User user = result.user;
    FirebaseService.analytics.logLogin(loginMethod: 'Email & Password');
    FirebaseService.crashlytics.setUserIdentifier(user.uid);
    return user;
  }

  Future registerWithEmailAndPassword(String email, String password) async {
    auth.UserCredential result = await _auth.createUserWithEmailAndPassword(
        email: email, password: password);
    auth.User user = result.user;
    FirebaseService.analytics.logSignUp(signUpMethod: 'Email & Password');
    FirebaseService.crashlytics.setUserIdentifier(user.uid);
    return user;
  }

  Future signOut() async {
    try {
      if (_firebaseUser.devices != null)
        _firebaseUser.devices.forEach((element) {
          FirebaseService.unsubscribeTopic(element.uid);
        });
      _firebaseUser = null;
      _currentUser = null;
      final SharedPreferences prefs = await SharedPreferences.getInstance();
      prefs.clear();
      return await _auth.signOut();
    } catch (e) {
      print(e.toString());
      return null;
    }
  }

  Future<String> getIdToken() {
    return _currentUser.getIdToken(true);
  }
}
