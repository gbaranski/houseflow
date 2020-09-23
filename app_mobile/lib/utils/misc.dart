String parseTotalSeconds(int totalSeconds) {
  return "${((totalSeconds / 60) % 60).floor()}m ${totalSeconds % 60}s";
}

String parseElapsedTotalSeconds(int totalSeconds) {
  return "${(totalSeconds / 3600).floor()}h ${parseTotalSeconds(totalSeconds)} ago";
}

String upperFirstCharacter(String name) {
  return name[0] + name.substring(1).toLowerCase();
}

Duration getEpochDiffDuration(int firstEpoch) {
  DateTime time = DateTime.fromMillisecondsSinceEpoch(firstEpoch);
  return time.difference(DateTime.now());
}

String durationToString(Duration duration) {
  int totalSeconds = duration.inSeconds.abs();

  return duration.isNegative
      ? parseElapsedTotalSeconds(totalSeconds)
      : parseTotalSeconds(totalSeconds);
}
