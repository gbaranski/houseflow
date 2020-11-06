import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:intl/intl.dart';

import 'additional_view.dart';

class InDepthDeviceHistory extends StatelessWidget {
  final DeviceHistory deviceHistory;
  static final TextStyle _dataTextStyle =
      TextStyle(fontSize: 15, color: Colors.black.withAlpha(180));

  InDepthDeviceHistory({@required this.deviceHistory});

  final DateFormat dateFormatter1 = DateFormat('dd MMMM y');
  final DateFormat dateFormatter2 = DateFormat('H:m:s');

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
                    deviceHistory.source.username,
                    style: _dataTextStyle,
                  ),
                ],
              ),
              Row(
                children: [
                  Icon(Icons.device_hub),
                  SizedBox(
                    width: 10,
                  ),
                  Text(
                    upperFirstCharacter(deviceHistory.destination.deviceType),
                    style: _dataTextStyle,
                  ),
                ],
              ),
              Row(
                children: [
                  Icon(Icons.today),
                  SizedBox(
                    width: 10,
                  ),
                  Text(
                    dateFormatter1.format(DateTime.fromMillisecondsSinceEpoch(
                        deviceHistory.timestamp)),
                    style: _dataTextStyle,
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
                    dateFormatter2.format(DateTime.fromMillisecondsSinceEpoch(
                        deviceHistory.timestamp)),
                    style: _dataTextStyle,
                  ),
                ],
              ),
              Row(
                children: [
                  Icon(Icons.public),
                  SizedBox(
                    width: 10,
                  ),
                  Text(
                    deviceHistory.source.ipAddress,
                    style: _dataTextStyle,
                  ),
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
                    Text(
                      deviceHistory.docUid,
                      style: _dataTextStyle,
                    ),
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
                getDeviceIcon(deviceRequest.destination.deviceType),
                color: Colors.blueGrey.shade500,
                size: 36,
              )
            ]),
            title: Text(deviceRequest.stringifyRequest()),
            subtitle: Text("${parseTimeAgo(deviceRequest.timestamp)} "),
          ),
        ),
      ),
    );
  }
}
