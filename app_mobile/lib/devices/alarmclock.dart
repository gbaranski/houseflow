import 'package:flutter/material.dart';

class _AlarmclockState extends State<Alarmclock> {
  @override
  Widget build(BuildContext context) {
    return Card(
      child: InkWell(
          splashColor: Colors.blue.withAlpha(30),
          onTap: () {
            print('Card tapped.');
          },
          child: Container(
            margin: const EdgeInsets.only(top: 10),
            child: Column(children: [
              Row(mainAxisAlignment: MainAxisAlignment.spaceEvenly, children: [
                Column(children: [
                  Text(
                    "Temperature",
                    style: TextStyle(
                        fontWeight: FontWeight.w300,
                        fontSize: 14,
                        color: Colors.black.withOpacity(0.6)),
                  ),
                  Row(
                    children: [
                      Icon(Icons.ac_unit),
                      Text("28,6°C",
                          style: TextStyle(
                              fontSize: 26, fontWeight: FontWeight.w300)),
                    ],
                  ),
                  Text(
                    "Alarm Time",
                    style: TextStyle(
                        fontWeight: FontWeight.w300,
                        fontSize: 14,
                        color: Colors.black.withOpacity(0.6)),
                  ),
                  Row(
                    children: [
                      Text("07:45",
                          style: TextStyle(
                              fontSize: 26, fontWeight: FontWeight.w300)),
                      Text("/OFF")
                    ],
                  )
                ]),
                Column(children: [
                  Text(
                    "Humidity",
                    style: TextStyle(
                        fontWeight: FontWeight.w300,
                        fontSize: 14,
                        color: Colors.black.withOpacity(0.6)),
                  ),
                  Row(
                    children: [
                      Icon(Icons.opacity),
                      Text("49%",
                          style: TextStyle(
                              fontSize: 26, fontWeight: FontWeight.w300)),
                    ],
                  ),
                  Text(
                    "Remaining Time",
                    style: TextStyle(
                        fontWeight: FontWeight.w300,
                        fontSize: 14,
                        color: Colors.black.withOpacity(0.6)),
                  ),
                  Text("09:54:02",
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
                        child: const Text('TEST ALARM'),
                        onPressed: () {/* ... */},
                      ),
                      FlatButton(
                        child: const Text('SET TIME'),
                        onPressed: () {/* ... */},
                      ),
                      FlatButton(
                        child: const Text('SWITCH STATE'),
                        onPressed: () {/* ... */},
                      ),
                    ],
                  ),
                ],
              )
            ]),
          )),
    );
  }
}

class Alarmclock extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _AlarmclockState();
  }
}
