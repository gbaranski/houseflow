String parseTotalSeconds(int totalSeconds) {
  return "${((totalSeconds / 60) % 60).floor()}m ${totalSeconds % 60}s";
}

String upperFirstCharacter(String name) {
  return name[0] + name.substring(1).toLowerCase();
}
