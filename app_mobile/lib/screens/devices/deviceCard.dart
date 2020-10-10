import 'package:flutter/material.dart';
import 'package:homeflow/shared/constants.dart';

class DeviceCard extends StatelessWidget {
  final List<Widget> children;
  DeviceCard({@required this.children});

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
        constraints: const BoxConstraints(minHeight: CardMinHeight),
        child: Card(
          child: InkWell(
            splashColor: Colors.blue.withAlpha(20),
            onTap: () => print("Card tapped"),
            child: Container(
              child: Column(
                children: children,
              ),
            ),
          ),
        ));
  }
}
