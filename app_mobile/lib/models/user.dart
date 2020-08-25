class FirebaseUser {
  final String uid;
  final String role;
  final List<dynamic> devices;

  final bool isAnonymous;

  FirebaseUser({this.uid, this.role, this.devices, this.isAnonymous});
}

enum AuthStatus {
  NOT_DETERMINED,
  NOT_LOGGED_IN,
  LOGGED_IN,
}
