import 'package:control_home/models/device.dart';
import 'package:control_home/models/devices/alarmclock.dart';
import 'package:control_home/models/devices/index.dart';
import 'package:control_home/services/device.dart';
import 'package:flutter/material.dart';
import 'package:control_home/shared/constants.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';
import 'package:provider/provider.dart';

class Alarmclock extends StatelessWidget {
  final String uid;

  Alarmclock({@required this.uid});

  @override
  Widget build(BuildContext context) {
    final deviceService = Provider.of<DeviceService>(context);

    final ActiveDevice activeDevice = deviceService.activeDevices
        .singleWhere((_device) => _device.uid == this.uid);

    final AlarmclockData data = AlarmclockData.fromJson(activeDevice.data);

    final switchAlarmState = () {
      final Map<String, dynamic> request = {
        "deviceUid": activeDevice.uid,
        "requestType": "SET_STATE",
        "data": {
          "state": data.alarmState == true ? false : true,
        }
      };
      print(deviceService.sendRequest(request));
    };

    final sendAlarmTime = (DeviceDateTime time) {
      final Map<String, dynamic> request = {
        "deviceUid": activeDevice.uid,
        "requestType": "SET_TIME",
        "data": {
          "hour": time.hour,
          "minute": time.minute,
          "second": time.second,
        },
      };
      print(deviceService.sendRequest(request));
    };

    return ConstrainedBox(
      constraints: BoxConstraints(minHeight: CardMinHeight),
      child: Card(
          child: InkWell(
        splashColor: Colors.blue.withAlpha(30),
        onTap: () {
          print('Card tapped.');
        },
        child: Container(
          child: Column(children: [
            SizedBox(
              height: 5,
            ),
            Text("Alarmclock", style: TextStyle(fontSize: 24)),
            Divider(
              indent: 20,
              endIndent: 20,
              thickness: 1,
            ),
            Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
              Column(children: [
                Text(
                  "Temperature",
                  style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 14,
                      color: Colors.black.withOpacity(0.6)),
                ),
                Row(children: [
                  Icon(
                    MdiIcons.thermometer,
                    color: Colors.black.withOpacity(0.75),
                  ),
                  Text("${data.sensor.temperature}°C",
                      style:
                          TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
                ]),
                Text(
                  "Alarm time",
                  style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 14,
                      color: Colors.black.withOpacity(0.6)),
                ),
                Row(children: [
                  Text(data.alarmTime.toReadableString(),
                      style:
                          TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
                  Text(data.alarmState ? "ON" : "OFF"),
                ])
              ]),
              Column(children: [
                Text(
                  "Humidity",
                  style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 14,
                      color: Colors.black.withOpacity(0.6)),
                ),
                Row(children: [
                  Icon(
                    MdiIcons.waterPercent,
                    color: Colors.black.withOpacity(0.75),
                    size: 26,
                  ),
                  Text("${data.sensor.humidity}%",
                      style:
                          TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
                ]),
                Text(
                  "Remaining time",
                  style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 14,
                      color: Colors.black.withOpacity(0.6)),
                ),
                Text(data.alarmTime.timeDiff(),
                    style:
                        TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
              ]),
            ]),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ButtonBar(
                  alignment: MainAxisAlignment.center,
                  children: <Widget>[
                    FlatButton(
                      child: const Text('SWITCH STATE'),
                      onPressed: () {
                        switchAlarmState();
                      },
                    ),
                    FlatButton(
                      child: const Text('SET TIME'),
                      onPressed: () async {
                        showTimePicker(
                                context: context, initialTime: TimeOfDay.now())
                            .then((date) {
                          sendAlarmTime(new DeviceDateTime(
                              hour: date.hour, minute: date.minute, second: 0));
                        });
                      },
                    ),
                  ],
                ),
              ],
            )
          ]),
        ),
      )),
    );
  }
}
