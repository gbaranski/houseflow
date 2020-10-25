import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/services.dart';
import 'package:intl/intl.dart';
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
  static DateFormat formatter = DateFormat('d MMMM y');
  var pickedDate = DateTime.now();

  @override
  void initState() {
    super.initState();
    widget.snapshotsStreams.forEach((stream) => stream.listen((snapshot) => {
          snapshot.docs.forEach((doc) {
            if (!deviceRequests.any((req) => req.docUid == doc.id)) {
              final DeviceRequest deviceRequest =
                  DeviceRequest.fromJson(doc.data(), doc.id);
              if (this.mounted) {
                setState(() {
                  deviceRequests.add(deviceRequest);
                  deviceRequests.sort((a, b) => b.timestamp - a.timestamp);
                });
              }
            }
          })
        }));
  }

  void setNextDay() {
    DateTime newDate = pickedDate.add(Duration(days: 1));
    setState(() {
      pickedDate = newDate;
    });
  }

  void setPreviousDay() {
    DateTime newDate = pickedDate.subtract(Duration(days: 1));
    setState(() {
      pickedDate = newDate;
    });
  }

  bool isAtSameDay(DateTime date, [DateTime secondDate]) {
    if (secondDate != null) {
      return date.day == secondDate.day &&
          date.month == secondDate.month &&
          date.year == secondDate.year;
    }

    final now = DateTime.now();
    return now.day == date.day &&
        now.month == date.month &&
        now.year == date.year;
  }

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      SizedBox(
        height: 20,
      ),
      Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: [
          IconButton(
              onPressed: setPreviousDay, icon: Icon(Icons.arrow_back_ios)),
          Text(formatter.format(pickedDate)),
          IconButton(
              onPressed: isAtSameDay(pickedDate) ? null : setNextDay,
              icon: Icon(Icons.arrow_forward_ios))
        ],
      ),
      SizedBox(
        height: 20,
      ),
      Column(
        children: !deviceRequests.any((deviceRequest) => isAtSameDay(
                DateTime.fromMillisecondsSinceEpoch(deviceRequest.timestamp),
                pickedDate))
            ? [Text("Nothing happened that day")]
            : deviceRequests.map((deviceRequest) {
                final DateTime rqDate = DateTime.fromMillisecondsSinceEpoch(
                    deviceRequest.timestamp);
                if (isAtSameDay(rqDate, pickedDate)) {
                  return SingleDeviceHistory(
                    deviceRequest: deviceRequest,
                  );
                } else {
                  return SizedBox();
                }
              }).toList(),
      )
    ]);
  }
}

class SingleDeviceHistory extends StatelessWidget {
  final DeviceRequest deviceRequest;

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
