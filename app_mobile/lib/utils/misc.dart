import 'package:flutter/material.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

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

String parseTimeAgo(int timestamp) {
  DateTime time = DateTime.fromMillisecondsSinceEpoch(timestamp);
  Duration diff = time.difference(DateTime.now()).abs();
  if (diff.inDays > 7) return "${diff.inDays} days ago";
  if (diff.inHours > 12) return "${diff.inHours}h ago";
  if (diff.inHours > 1)
    return "${diff.inHours}h ${diff.inMinutes % 60 - diff.inHours}m ago";
  if (diff.inMinutes > 1) return "few seconds ago";
  return '${diff.inMinutes}m ago';
}

IconData getDeviceIcon(String deviceType) {
  switch (deviceType) {
    case 'WATERMIXER':
      return Icons.hot_tub;
    case 'GATE':
      return MdiIcons.gate;
    case 'GARAGE':
      return MdiIcons.garage;
    default:
      return Icons.error;
  }
}
