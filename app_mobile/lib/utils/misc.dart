import 'dart:math';

import 'package:timeago/timeago.dart' as timeago;
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
  return timeago.format(time);
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

String createNonce(int length) {
  final random = Random();
  final charCodes = List<int>.generate(length, (_) {
    int codeUnit;

    switch (random.nextInt(3)) {
      case 0:
        codeUnit = random.nextInt(10) + 48;
        break;
      case 1:
        codeUnit = random.nextInt(26) + 65;
        break;
      case 2:
        codeUnit = random.nextInt(26) + 97;
        break;
    }

    return codeUnit;
  });

  return String.fromCharCodes(charCodes);
}

bool isAtSameDay(DateTime date, [DateTime secondDate]) {
  if (secondDate != null) {
    return date.day == secondDate.day &&
        date.month == secondDate.month &&
        date.year == secondDate.year;
  }

  final now = DateTime.now();
  return now.day == date.day &&
      now.month == date.month &&
      now.year == date.year;
}
