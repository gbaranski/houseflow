import 'package:flutter/material.dart';
import 'dart:math';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/relay.dart';
import 'package:houseflow/screens/single_device/single_device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:mqtt_client/mqtt_client.dart';

class RelayCard extends StatefulWidget {
  final FirebaseDevice firebaseDevice;
  final Color cardColor;
  final IconData iconData;
  final int Function() getNewDeviceData;

  RelayCard({
    @required this.firebaseDevice,
    @required this.cardColor,
    @required this.iconData,
    @required this.getNewDeviceData,
  });

  @override
  _RelayCardState createState() => _RelayCardState();
}

class _RelayCardState extends State<RelayCard>
    with SingleTickerProviderStateMixin {
  AnimationController _animationController;
  Animation<double> _animation;
  AnimationStatus _animationStatus = AnimationStatus.dismissed;

  void sendRelaySignal() {
    print("MQTT CONN STAT: ${MqttService.mqttClient.connectionStatus}");
    if (MqttService.mqttClient.connectionStatus.state !=
        MqttConnectionState.connected) {
      const snackbar = SnackBar(
        content: Text("Error! Not connected to the server, please restart app"),
        duration: Duration(milliseconds: 1500),
      );
      Scaffold.of(context).showSnackBar(snackbar);
      return;
    }
    final String uid = widget.firebaseDevice.uid;
    final DeviceTopic topic = RelayData.getSendSignalTopic(uid);

    bool hasCompleted = false;
    final Future req = MqttService.sendMessage(
        topic: topic, qos: MqttQos.atMostOnce, data: null);

    req.whenComplete(() {
      hasCompleted = true;
      const snackbar = SnackBar(
        content: Text("Success sending!"),
        duration: Duration(milliseconds: 500),
      );
      Scaffold.of(context).hideCurrentSnackBar();
      Scaffold.of(context).showSnackBar(snackbar);
      final RelayData newDeviceData =
          RelayData(lastSignalTimestamp: widget.getNewDeviceData());
      FirebaseService.updateFirebaseDeviceData(uid, newDeviceData.toJson());
    });
    Future.delayed(Duration(seconds: 2), () {
      if (!hasCompleted) {
        const snackbar = SnackBar(content: Text("No response from device!"));
        Scaffold.of(context).showSnackBar(snackbar);
      }
    });
  }

  void triggerAnimation() {
    if (_animationStatus == AnimationStatus.dismissed)
      _animationController.forward();
    else
      _animationController.reverse();
  }

  void showDeviceOfflineSnackbar() {
    const snackbar = SnackBar(
      content: Text("Device is offline!"),
      duration: Duration(milliseconds: 600),
    );
    Scaffold.of(context).showSnackBar(snackbar);
  }

  Widget offlineCardBanner({@required Widget child}) {
    if (widget.firebaseDevice.status) {
      return child;
    } else {
      return Banner(
        location: BannerLocation.topStart,
        message: "OFFLINE",
        child: child,
      );
    }
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
        onTap: widget.firebaseDevice.status
            ? triggerAnimation
            : showDeviceOfflineSnackbar,
        child: ClipRRect(
          child: offlineCardBanner(
            child: Container(
              margin: const EdgeInsets.all(10),
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
                  ),
                ],
              ),
            ),
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
                    onPressed: () {
                      sendRelaySignal();
                      triggerAnimation();
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
        width: 180,
        child: _animation.value <= 0.5 ? basicCard() : confirmationCard(),
      ),
    );
  }
}
