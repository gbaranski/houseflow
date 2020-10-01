import 'package:homeflow/models/device.dart';
import 'package:homeflow/models/devices/watermixer.dart';
import 'package:flutter/material.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:mqtt_client/mqtt_server_client.dart';
import 'package:homeflow/services/mqtt.dart';
import 'package:homeflow/utils/misc.dart';
import 'package:provider/provider.dart';
import 'package:homeflow/shared/constants.dart';
import 'dart:async';

class Watermixer extends StatefulWidget {
  final FirebaseDevice firebaseDevice;

  Watermixer({@required this.firebaseDevice});

  @override
  _WatermixerState createState() => _WatermixerState();
}

class _WatermixerState extends State<Watermixer> {
  Timer _countdownTimer;
  String mixingTimeString = "";

  void startCounting(int finishMixTimestamp) {
    final callback = (Timer timer) => setState(() {
          mixingTimeString =
              durationToString(getEpochDiffDuration(finishMixTimestamp));
        });

    _countdownTimer = Timer.periodic(Duration(seconds: 1), callback);
    callback(_countdownTimer);
  }

  @override
  void dispose() {
    _countdownTimer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final MqttService mqttService =
        Provider.of<MqttService>(context, listen: false);
    final WatermixerData data =
        WatermixerData.fromJson(widget.firebaseDevice.data);
    startCounting(data.finishMixTimestamp);

    final startMixing = () {
      print("MQTT CONN STAT: ${mqttService.mqttClient.connectionStatus}");
      final String uid = widget.firebaseDevice.uid;
      final RequestTopic topic = RequestTopic(
          request: '$uid/event/startmix/request',
          response: '$uid/event/startmix/request');

      mqttService.sendMessage(
          topic: topic, qos: MqttQos.atMostOnce, data: null);
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
            Text("Watermixer", style: TextStyle(fontSize: 24)),
            Divider(
              indent: 20,
              endIndent: 20,
              thickness: 1,
            ),
            Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
              Column(children: [
                Text(
                  "Mixing state",
                  style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 14,
                      color: Colors.black.withOpacity(0.6)),
                ),
                Text(
                    data.finishMixTimestamp >
                            DateTime.now().millisecondsSinceEpoch
                        ? "Mixing!"
                        : "Idle",
                    style:
                        TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
              ]),
              Column(children: [
                Text(
                  "Mixing time",
                  style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 14,
                      color: Colors.black.withOpacity(0.6)),
                ),
                Text(mixingTimeString,
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
                      child: const Text('START MIXING'),
                      onPressed: () {
                        startMixing();
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
