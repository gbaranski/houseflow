import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/gate.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/screens/devices/deviceCard.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:houseflow/shared/constants.dart';
import 'dart:async';

const tenMinutesInMillis = 1000 * 10 * 60;

class Gate extends StatefulWidget {
  final FirebaseDevice firebaseDevice;

  Gate({@required this.firebaseDevice});

  @override
  _GateState createState() => _GateState();
}

class _GateState extends State<Gate> {
  Timer _countdownTimer;
  String lastOpenString = "";

  void startCounting(int lastOpenTimestamp) {
    final callback = (Timer timer) {
      if (!this.mounted) {
        timer.cancel();
        return;
      }
      setState(() {
        lastOpenString =
            durationToString(getEpochDiffDuration(lastOpenTimestamp));
      });
    };

    _countdownTimer = Timer.periodic(Duration(seconds: 1), callback);
    callback(_countdownTimer);
  }

  @override
  void dispose() {
    _countdownTimer.cancel();
    _countdownTimer = null;
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final GateData data = GateData.fromJson(widget.firebaseDevice.data);
    startCounting(data.lastOpenTimestamp);

    final openGate = () async {
      print("MQTT CONN STAT: ${MqttService.mqttClient.connectionStatus}");
      final String uid = widget.firebaseDevice.uid;

      final DeviceTopic topic = GateData.getOpenGateTopic(uid);
      bool hasCompleted = false;
      final Future req = MqttService.sendMessage(
          topic: topic, qos: MqttQos.atMostOnce, data: null);

      req.whenComplete(() {
        hasCompleted = true;
        const snackbar = SnackBar(
          content: Text("Success opening gate!"),
          duration: Duration(milliseconds: 500),
        );
        Scaffold.of(context).showSnackBar(snackbar);
        final GateData newDeviceData =
            GateData(lastOpenTimestamp: DateTime.now().millisecondsSinceEpoch);
        FirebaseService.updateFirebaseDeviceData(uid, newDeviceData.toJson());
      });
      Future.delayed(Duration(seconds: 2), () {
        if (!hasCompleted) {
          const snackbar = SnackBar(content: Text("No response from device!"));
          Scaffold.of(context).showSnackBar(snackbar);
        }
      });
    };

    return DeviceCard(children: [
      SizedBox(
        height: 5,
      ),
      Text("Gate", style: TextStyle(fontSize: 24)),
      Divider(
        indent: 20,
        endIndent: 20,
        thickness: 1,
      ),
      Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
        Column(children: [
          Text(
            "Last time open",
            style: TextStyle(
                fontWeight: FontWeight.w300,
                fontSize: 14,
                color: Colors.black.withOpacity(0.6)),
          ),
          Text(lastOpenString,
              style: TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
        ]),
      ]),
      Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          ButtonBar(
            alignment: MainAxisAlignment.center,
            children: <Widget>[
              OutlineButton(
                borderSide: BorderSide(
                    color: LayoutBlueColor1.withAlpha(100), width: 0.5),
                shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(10)),
                child: const Text('OPEN GATE'),
                onPressed: () {
                  openGate();
                },
              ),
            ],
          ),
        ],
      )
    ]);
  }
}
