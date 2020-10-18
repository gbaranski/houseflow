import 'package:flutter/material.dart';
import 'dart:math';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/screens/single_device/single_device.dart';
import 'package:houseflow/utils/misc.dart';

class RelayCard extends StatefulWidget {
  final FirebaseDevice firebaseDevice;
  final Color cardColor;
  final IconData iconData;

  RelayCard(
      {@required this.firebaseDevice,
      @required this.cardColor,
      @required this.iconData});

  @override
  _RelayCardState createState() => _RelayCardState();
}

class _RelayCardState extends State<RelayCard>
    with SingleTickerProviderStateMixin {
  AnimationController _animationController;
  Animation<double> _animation;
  AnimationStatus _animationStatus = AnimationStatus.dismissed;

  void triggerAnimation() {
    if (_animationStatus == AnimationStatus.dismissed)
      _animationController.forward();
    else
      _animationController.reverse();
  }

  Widget basicCard() {
    return Card(
      color: widget.cardColor,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20.0)),
      child: InkWell(
        onLongPress: () => Navigator.push(
            context,
            MaterialPageRoute(
                builder: (context) =>
                    SingleDevice(firebaseDevice: widget.firebaseDevice))),
        splashColor: Colors.white.withAlpha(100),
        onTap: triggerAnimation,
        child: Container(
          margin: EdgeInsets.all(10),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Icon(
                widget.iconData,
                color: Colors.white.withAlpha(180),
                size: 72,
              ),
              Text(
                upperFirstCharacter(widget.firebaseDevice.type),
                style: TextStyle(
                    color: Colors.white.withAlpha(190),
                    fontSize: 20,
                    fontWeight: FontWeight.w100),
              )
            ],
          ),
        ),
      ),
    );
  }

  Widget confirmationCard() {
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
                    onPressed: triggerAnimation,
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
                    onPressed: triggerAnimation,
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

  @override
  void initState() {
    super.initState();
    _animationController =
        AnimationController(vsync: this, duration: Duration(milliseconds: 300));
    _animation = Tween<double>(end: 1, begin: 0).animate(_animationController)
      ..addListener(() {
        setState(() {});
      })
      ..addStatusListener((status) {
        _animationStatus = status;
      });
  }

  @override
  void dispose() {
    super.dispose();
    _animationController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Transform(
      alignment: FractionalOffset.center,
      transform: Matrix4.identity()
        ..setEntry(3, 2, 0.002)
        ..rotateX(pi * _animation.value),
      child: Container(
        height: 140,
        width: 200,
        child: _animation.value <= 0.5 ? basicCard() : confirmationCard(),
      ),
    );
  }
}
