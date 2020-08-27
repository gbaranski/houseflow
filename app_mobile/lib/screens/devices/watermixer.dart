import 'package:app_mobile/models/device.dart';
import 'package:app_mobile/models/devices/watermixer.dart';
import 'package:flutter/material.dart';
import 'package:app_mobile/utils/misc.dart';

class Watermixer extends StatelessWidget {
  final ActiveDevice activeDevice;

  Watermixer({@required this.activeDevice});

  @override
  Widget build(BuildContext context) {
    final WatermixerData data = WatermixerData.fromJson(activeDevice.data);

    return Card(
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
          SizedBox(
            height: 5,
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
              Text(data.isTimerOn ? "Mixing!" : "Idle",
                  style: TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
            ]),
            Column(children: [
              Text(
                "Remaining Time",
                style: TextStyle(
                    fontWeight: FontWeight.w300,
                    fontSize: 14,
                    color: Colors.black.withOpacity(0.6)),
              ),
              Text(parseTotalSeconds(data.remainingSeconds).toString(),
                  style: TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
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
                    onPressed: () {/* ... */},
                  ),
                ],
              ),
            ],
          )
        ]),
      ),
    ));
  }
}
