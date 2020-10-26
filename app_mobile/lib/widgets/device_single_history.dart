import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:intl/intl.dart';

import 'additional_view.dart';

class InDepthDeviceHistory extends StatelessWidget {
  final DeviceHistory deviceHistory;
  InDepthDeviceHistory({@required this.deviceHistory});

  final DateFormat formatter = DateFormat('yyyy-MM-dd');

  void copyDocumentUidToClipboard(BuildContext context) {
    Clipboard.setData(ClipboardData(text: deviceHistory.docUid)).then((_) {
      HapticFeedback.vibrate();
      const snackBar = SnackBar(
        content: Text("Document UID copied to clipboard"),
      );
      Scaffold.of(context).showSnackBar(snackBar);
    });
  }

  @override
  Widget build(BuildContext context) {
    return AdditionalView(
      body: Builder(builder: (context) {
        return Container(
          margin: const EdgeInsets.only(left: 20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                deviceHistory.stringifyRequest(),
                style: TextStyle(fontSize: 32, fontWeight: FontWeight.w600),
              ),
              Row(
                children: [
                  Icon(Icons.person),
                  SizedBox(
                    width: 10,
                  ),
                  Text(
                    deviceHistory.username,
                    style: TextStyle(fontSize: 18),
                  ),
                ],
              ),
              Row(
                children: [
                  Icon(Icons.schedule),
                  SizedBox(
                    width: 10,
                  ),
                  Text(
                    formatter.format(DateTime.fromMillisecondsSinceEpoch(
                        deviceHistory.timestamp)),
                  ),
                ],
              ),
              Row(
                children: [
                  Icon(Icons.public),
                  SizedBox(
                    width: 10,
                  ),
                  Text(deviceHistory.ipAddress),
                ],
              ),
              GestureDetector(
                onLongPress: () => copyDocumentUidToClipboard(context),
                child: Row(
                  children: [
                    Icon(Icons.source),
                    SizedBox(
                      width: 10,
                    ),
                    Text(deviceHistory.docUid),
                  ],
                ),
              ),
            ],
          ),
        );
      }),
    );
  }
}

class SingleDeviceHistory extends StatelessWidget {
  final DeviceHistory deviceRequest;

  const SingleDeviceHistory({
    Key key,
    @required this.deviceRequest,
  }) : super(key: key);

  void navigateToInDepthInfo(BuildContext context) {
    Navigator.push(
        context,
        MaterialPageRoute(
            settings: RouteSettings(name: 'InDepthDeviceHistory'),
            builder: (context) => InDepthDeviceHistory(
                  deviceHistory: deviceRequest,
                )));
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: EdgeInsets.symmetric(vertical: 5),
      child: Card(
        elevation: 0.3,
        margin: EdgeInsets.symmetric(horizontal: 20),
        child: InkWell(
          onTap: () => navigateToInDepthInfo(context),
          child: ListTile(
            leading:
                Column(mainAxisAlignment: MainAxisAlignment.center, children: [
              Icon(
                getDeviceIcon(deviceRequest.deviceType),
                color: Colors.blueGrey.shade500,
                size: 36,
              )
            ]),
            title: Text(deviceRequest.stringifyRequest()),
            subtitle: Text("${parseTimeAgo(deviceRequest.timestamp)} "),
            // child: Row(
            //   children: [
            //     Column(
            //       mainAxisAlignment: MainAxisAlignment.center,
            //       children: [
            //         Icon(
            //           getDeviceIcon(deviceRequest.deviceType),
            //           size: 40,
            //           color: Colors.blueGrey,
            //         )
            //       ],
            //     ),
            //     SizedBox(
            //       width: 20,
            //     ),
            //     Column(
            //       crossAxisAlignment: CrossAxisAlignment.start,
            //       children: [
            //         Row(
            //           children: [
            //             Text(
            //               "${parseTimeAgo(deviceRequest.timestamp)} · ${deviceRequest.username}",
            //               style: TextStyle(color: Colors.black45, fontSize: 12),
            //             )
            //           ],
            //         ),
            //         Row(
            //           children: [
            //             Text(
            //               "${deviceRequest.stringifyRequest(deviceRequest.deviceType)}",
            //               style: TextStyle(
            //                   fontSize: 15, color: Colors.blueGrey.shade800),
            //             )
            //           ],
            //         )
            //       ],
            //     )
            // ],
            // ),
          ),
        ),
      ),
    );
  }
}
