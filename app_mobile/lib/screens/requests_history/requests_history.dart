import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/user.dart';
import 'package:houseflow/utils/misc.dart';

class RequestsHistory extends StatefulWidget {
  final List<FirebaseUserDevice> firebaseUserDevices;
  final List<Stream<QuerySnapshot>> snapshotsStreams;

  RequestsHistory(
      {@required this.firebaseUserDevices, @required this.snapshotsStreams});

  @override
  _RequestsHistoryState createState() => _RequestsHistoryState();
}

class _RequestsHistoryState extends State<RequestsHistory> {
  final List<DeviceRequest> deviceRequests = [];

  @override
  void initState() {
    super.initState();
    widget.snapshotsStreams.forEach((stream) => stream.listen((snapshot) => {
          snapshot.docs.forEach((doc) {
            if (!deviceRequests.any((req) => req.docUid == doc.id)) {
              final DeviceRequest deviceRequest =
                  DeviceRequest.fromJson(doc.data(), doc.id);
              setState(() {
                deviceRequests.add(deviceRequest);
              });
            }
          })
        }));
  }

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: deviceRequests.length,
      itemBuilder: (context, index) {
        return SingleDeviceHistory(
          deviceRequest: deviceRequests[index],
        );
      },
    );
  }
}

class SingleDeviceHistory extends StatelessWidget {
  final DeviceRequest deviceRequest;

  const SingleDeviceHistory({
    Key key,
    @required this.deviceRequest,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.symmetric(horizontal: 20),
      child: Container(
        margin: EdgeInsets.all(20),
        height: 40,
        child: Row(
          children: [
            Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(
                  Icons.sms,
                  color: Colors.blueGrey,
                  size: 40,
                ),
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
                      "${parseTimeAgo(deviceRequest.timestamp)} ·",
                      style: TextStyle(color: Colors.black45, fontSize: 12),
                    )
                  ],
                ),
                Row(
                  children: [
                    Text(
                      deviceRequest.stringifyRequest(deviceRequest.deviceType),
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
    );
  }
}
