import 'dart:io';
import 'package:crypto/crypto.dart';
import 'dart:convert';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/models/user.dart';
import 'package:firebase_auth/firebase_auth.dart' as auth;
import 'package:google_sign_in/google_sign_in.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:sign_in_with_apple/sign_in_with_apple.dart';

class AuthService {
  static final auth.FirebaseAuth _auth = auth.FirebaseAuth.instance;
  static auth.User currentUser;
  static FirebaseUser firebaseUser;

  static AuthStatus authStatus = AuthStatus.NOT_DETERMINED;

  // auth change user stream
  static Stream<auth.User> get user {
    return _auth.authStateChanges();
  }

  static Future signInAnon() async {
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

  static Future<auth.UserCredential> signInWithGoogle() async {
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

  static Future<auth.UserCredential> signInWithApple() async {
    final oauthCred = await _createAppleOAuthCred();
    final credentials = await _auth.signInWithCredential(oauthCred);
    if (credentials.additionalUserInfo.isNewUser)
      FirebaseService.analytics.logSignUp(signUpMethod: 'Apple');
    else
      FirebaseService.analytics.logLogin(loginMethod: 'Apple');

    FirebaseService.crashlytics.setUserIdentifier(credentials.user.uid);
    return credentials;
  }

  static Future signInWithEmailAndPassword(
      String email, String password) async {
    auth.UserCredential result = await _auth.signInWithEmailAndPassword(
        email: email, password: password);
    auth.User user = result.user;
    FirebaseService.analytics.logLogin(loginMethod: 'Email & Password');
    FirebaseService.crashlytics.setUserIdentifier(user.uid);
    return user;
  }

  static Future registerWithEmailAndPassword(
      String email, String password) async {
    auth.UserCredential result = await _auth.createUserWithEmailAndPassword(
        email: email, password: password);
    auth.User user = result.user;
    FirebaseService.analytics.logSignUp(signUpMethod: 'Email & Password');
    FirebaseService.crashlytics.setUserIdentifier(user.uid);
    return user;
  }

  static Future signOut() async {
    try {
      if (firebaseUser.devices != null)
        firebaseUser.devices.forEach((element) {
          FirebaseService.unsubscribeTopic(element.uid);
        });
      firebaseUser = null;
      currentUser = null;
      return await _auth.signOut();
    } catch (e) {
      print(e.toString());
      return null;
    }
  }

  static Future<String> getIdToken() {
    return currentUser.getIdToken(true);
  }
}
