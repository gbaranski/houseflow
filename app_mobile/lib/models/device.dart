import 'package:app_mobile/models/devices/alarmclock.dart';
import 'package:flutter/material.dart';

class FirebaseDevice {
  String uid;
  String type;

  FirebaseDevice({@required this.uid, @required this.type});
}

class ActiveDevice<DataType extends AlarmclockData, WatermixerData, Dynamic>
    extends FirebaseDevice {
  String ip;
  DataType data;

  ActiveDevice({@required ip, @required data});
}
