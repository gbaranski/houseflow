import 'package:app_mobile/models/device.dart';
import 'package:app_mobile/models/devices/watermixer.dart';
import 'package:app_mobile/services/device.dart';
import 'package:flutter/material.dart';
import 'package:app_mobile/utils/misc.dart';
import 'package:app_mobile/shared/constants.dart';
import 'package:provider/provider.dart';

class Watermixer extends StatelessWidget {
  final String uid;

  Watermixer({@required this.uid});

  @override
  Widget build(BuildContext context) {
    final deviceService = Provider.of<DeviceService>(context);

    final ActiveDevice activeDevice = deviceService.activeDevices
        .singleWhere((_device) => _device.uid == this.uid);

    final WatermixerData data = WatermixerData.fromJson(activeDevice.data);

    final startMixing = () {
      final Map<String, dynamic> request = {
        "deviceUid": activeDevice.uid,
        "requestType": "START_MIXING",
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
                Text(data.isTimerOn ? "Mixing!" : "Idle",
                    style:
                        TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
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
