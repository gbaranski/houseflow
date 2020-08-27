String parseTotalSeconds(int totalSeconds) {
  return "${((totalSeconds / 60) % 60).floor()}m ${totalSeconds % 60}s";
}
