import 'package:flutter/material.dart';

class CardTitle extends StatelessWidget {
  final String cardName;

  const CardTitle({Key key, this.cardName}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 5),
      child: Text(
        this.cardName,
        style: TextStyle(fontSize: 24, color: Colors.black.withOpacity(0.6)),
      ),
    );
  }
}
