import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/utils/misc.dart';

class RequestsHistory extends StatelessWidget {
  Widget singleHistory(
      DeviceRequest deviceRequest, FirebaseDevice firebaseDevice) {
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
                      "${parseTimeAgo(deviceRequest.timestamp)} · Watermixer",
                      style: TextStyle(color: Colors.black45, fontSize: 12),
                    )
                  ],
                ),
                Row(
                  children: [
                    Text(
                      deviceRequest.stringifyRequest(firebaseDevice.type),
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

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        SizedBox(
          height: 20,
        ),
        singleHistory(deviceRequest, firebaseDevice),
      ],
    );
  }
}
