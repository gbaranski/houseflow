import 'package:flutter/material.dart';
import 'dart:math';

class ConfirmationCard extends StatelessWidget {
  final Function() onConfirm;
  final Function() onCancel;
  ConfirmationCard({@required this.onConfirm, @required this.onCancel});

  @override
  Widget build(BuildContext context) {
    return Transform(
      alignment: Alignment.center,
      transform: Matrix4.rotationX(pi),
      child: Card(
        shape:
            RoundedRectangleBorder(borderRadius: BorderRadius.circular(20.0)),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Expanded(
              child: Ink(
                decoration: BoxDecoration(
                  color: Colors.green,
                  borderRadius:
                      BorderRadius.horizontal(left: Radius.circular(20)),
                ),
                child: IconButton(
                    onPressed: () {
                      onConfirm();
                      // widget.onSubmit(context);
                      // triggerAnimation();
                    },
                    splashRadius: 100,
                    splashColor: Colors.green,
                    highlightColor: Colors.green.withAlpha(100),
                    icon: Icon(Icons.done, color: Colors.white, size: 36)),
              ),
            ),
            Expanded(
              child: Ink(
                decoration: BoxDecoration(
                  color: Colors.red,
                  borderRadius:
                      BorderRadius.horizontal(right: Radius.circular(20)),
                ),
                child: IconButton(
                    onPressed: onCancel,
                    splashRadius: 100,
                    splashColor: Colors.red.withAlpha(180),
                    highlightColor: Colors.red.withAlpha(50),
                    icon: Icon(Icons.close, color: Colors.white, size: 36)),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
