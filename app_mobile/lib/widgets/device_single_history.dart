import 'package:flutter/services.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/utils/misc.dart';

class SingleDeviceHistory extends StatelessWidget {
  final DeviceHistory deviceRequest;

  const SingleDeviceHistory({
    Key key,
    @required this.deviceRequest,
  }) : super(key: key);

  void copyDocUid(BuildContext context) {
    Clipboard.setData(ClipboardData(text: deviceRequest.docUid)).then((_) {
      HapticFeedback.vibrate();
      final snackBar = SnackBar(
        content: Text("${deviceRequest.docUid} copied to clipboard"),
        duration: Duration(milliseconds: 500),
      );
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: EdgeInsets.symmetric(vertical: 5),
      child: Card(
        elevation: 0.05,
        margin: EdgeInsets.symmetric(horizontal: 20),
        child: InkWell(
          onTap: () {},
          onLongPress: () => copyDocUid(context),
          child: Container(
            margin: EdgeInsets.all(20),
            height: 40,
            child: Row(
              children: [
                Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      getDeviceIcon(deviceRequest.deviceType),
                      size: 40,
                      color: Colors.blueGrey,
                    )
                  ],
                ),
                SizedBox(
                  width: 20,
                ),
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Text(
                          "${parseTimeAgo(deviceRequest.timestamp)} · ${upperFirstCharacter(deviceRequest.deviceType)} · ${deviceRequest.ipAddress}",
                          style: TextStyle(color: Colors.black45, fontSize: 12),
                        )
                      ],
                    ),
                    Row(
                      children: [
                        Text(
                          deviceRequest
                              .stringifyRequest(deviceRequest.deviceType),
                          style: TextStyle(
                              fontSize: 15, color: Colors.blueGrey.shade800),
                        )
                      ],
                    )
                  ],
                )
              ],
            ),
          ),
        ),
      ),
    );
  }
}
