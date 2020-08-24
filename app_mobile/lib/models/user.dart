import 'package:cloud_firestore/cloud_firestore.dart';

class FirebaseUser {
  final String uid;
  final String role;
  final List<DocumentReference> devices;

  final bool isAnonymous;

  FirebaseUser({this.uid, this.role, this.devices, this.isAnonymous});
}
