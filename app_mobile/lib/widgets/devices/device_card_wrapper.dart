import 'package:flutter/material.dart';
import 'dart:math';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/services/mqtt.dart';
import 'package:houseflow/widgets/devices/confirmation_card..dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:provider/provider.dart';

import 'device_card.dart';

class DeviceCardWrapper extends StatefulWidget {
  final IconData iconData;
  final Color color;
  final FirebaseDevice firebaseDevice;
  final DeviceTopic Function(String uid) getDeviceTopic;
  final Map<String, dynamic> Function() getNewDeviceData;

  DeviceCardWrapper(
      {@required this.iconData,
      @required this.color,
      @required this.firebaseDevice,
      @required this.getDeviceTopic,
      @required this.getNewDeviceData});

  @override
  _DeviceCardWrapperState createState() => _DeviceCardWrapperState();
}

class _DeviceCardWrapperState extends State<DeviceCardWrapper>
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

  void onNotConnectedToMqttWhenSending(
      BuildContext context, MqttConnectionState mqttConnectionState) {
    const snackbar = SnackBar(
      content:
          Text("Error! Not connected to the server, please try restarting app"),
      duration: Duration(milliseconds: 1500),
    );
    FirebaseService.crashlytics.recordError(
        "mqtt_not_connected_when_sending_request", StackTrace.current,
        information: [
          DiagnosticsNode.message(
            "DEVICE_UID: ${widget.firebaseDevice.uid}",
          ),
          DiagnosticsNode.message("CONNSTATE: $mqttConnectionState"),
        ]);
    Scaffold.of(context).showSnackBar(snackbar);
    return;
  }

  void onSucess(BuildContext context) {
    const snackbar = SnackBar(
      content: Text("Success!"),
      duration: Duration(milliseconds: 500),
    );
    Scaffold.of(context).hideCurrentSnackBar();
    Scaffold.of(context).showSnackBar(snackbar);
    FirebaseService.updateFirebaseDeviceData(
        widget.firebaseDevice.uid, widget.getNewDeviceData());
    FirebaseService.analytics.logEvent(name: 'device_action', parameters: {
      'type': widget.firebaseDevice.type.toLowerCase(),
      'uid': widget.firebaseDevice.uid,
      'request': 'sendSignal',
    });
  }

  void onMessageTimedOut(BuildContext context) {
    const snackbar = SnackBar(
      content: Text("Could not connect to device"),
    );
    Scaffold.of(context).showSnackBar(snackbar);
  }

  void onSubmit(BuildContext context) {
    triggerAnimation();
    final MqttService mqttService =
        Provider.of<MqttService>(context, listen: false);

    final DeviceTopic topic = widget.getDeviceTopic(widget.firebaseDevice.uid);

    final req = mqttService.sendMessage(
        topic: topic, qos: MqttQos.atMostOnce, data: null);

    req.then((sendMessageStatus) {
      switch (sendMessageStatus) {
        case SendMessageStatus.not_connected_to_mqtt:
          return onNotConnectedToMqttWhenSending(
              context, mqttService.mqttClient.connectionStatus.state);
        case SendMessageStatus.timed_out:
          return onMessageTimedOut(context);
        case SendMessageStatus.success:
          return onSucess(context);
      }
    });
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
        child: _animation.value <= 0.5
            ? DeviceCard(
                color: widget.color,
                iconData: widget.iconData,
                firebaseDevice: widget.firebaseDevice,
                onValidTap: () {
                  triggerAnimation();
                },
              )
            : ConfirmationCard(
                onCancel: triggerAnimation,
                onConfirm: () => onSubmit(context),
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
}
