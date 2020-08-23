import 'package:flutter/material.dart';

class _WatermixerState extends State<Watermixer> {
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
          Container(
            margin: const EdgeInsets.only(bottom: 5),
            child: Text(
              "Watermixer",
              style:
                  TextStyle(fontSize: 24, color: Colors.black.withOpacity(0.6)),
            ),
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
              Row(
                children: [
                  Text("Mixing!",
                      style:
                          TextStyle(fontSize: 26, fontWeight: FontWeight.w300)),
                ],
              ),
            ]),
            Column(children: [
              Text(
                "Remaining Time",
                style: TextStyle(
                    fontWeight: FontWeight.w300,
                    fontSize: 14,
                    color: Colors.black.withOpacity(0.6)),
              ),
              Text("9m 12s",
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

class Watermixer extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _WatermixerState();
  }
}
