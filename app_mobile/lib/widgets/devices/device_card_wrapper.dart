import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';
import 'dart:math';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/services/auth.dart';
import 'package:houseflow/services/device.dart';
import 'package:houseflow/services/firebase.dart';
import 'package:houseflow/utils/misc.dart';
import 'package:provider/provider.dart';
import 'confirmation_card.dart';
import 'device_card.dart';

class DeviceCardWrapper extends StatefulWidget {
  final Color color;
  final FirebaseDevice firebaseDevice;
  final DeviceRequestDevice deviceRequestDevice;
  final Function onSuccessCallback;

  DeviceCardWrapper(
      {@required this.color,
      @required this.firebaseDevice,
      @required this.deviceRequestDevice,
      this.onSuccessCallback});

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

  void onSucess(BuildContext context, DeviceRequest deviceRequest) {
    if (widget.onSuccessCallback != null) widget.onSuccessCallback();
    const snackbar = SnackBar(
      content: Text("Success!"),
      duration: Duration(milliseconds: 500),
    );
    Scaffold.of(context).hideCurrentSnackBar();
    Scaffold.of(context).showSnackBar(snackbar);
    FirebaseService.analytics.logEvent(name: 'device_action', parameters: {
      'type': widget.firebaseDevice.type.toLowerCase(),
      'uid': widget.firebaseDevice.uid,
      'request': deviceRequest.device.toString(),
    });
  }

  void onMessageTimedOut(BuildContext context) {
    const snackbar = SnackBar(
      content: Text("Could not connect to device"),
    );
    Scaffold.of(context).showSnackBar(snackbar);
  }

  static Widget alertDialog(BuildContext context) => AlertDialog(
        title: const Text("Accept location services"),
        content: const SingleChildScrollView(
          child: Text(
              "Please accept location services, we use them only to guarantee device security"),
        ),
        actions: [
          TextButton(
            child: Text("OK"),
            onPressed: () {
              Geolocator.requestPermission();
              Navigator.of(context).pop();
            },
          )
        ],
      );

  Future<GeoPoint> getCurrentGeoPoint(BuildContext context) async {
    try {
      Position position = await Geolocator.getLastKnownPosition();
      if (position == null)
        position = await Geolocator.getCurrentPosition(
            desiredAccuracy: LocationAccuracy.high);
      return GeoPoint(
          latitude: position.latitude, longitude: position.longitude);
    } catch (e) {
      print(e);
      showDialog(context: context, builder: (context) => alertDialog(context));
      return null;
    }
  }

  void onSubmit(BuildContext context,
      [bool shouldTriggerAnimation = true]) async {
    if (shouldTriggerAnimation) triggerAnimation();
    final GeoPoint geoPoint = await getCurrentGeoPoint(context);
    if (geoPoint == null) return;

    final authService = Provider.of<AuthService>(context, listen: false);
    final token = await authService.getIdToken();
    final DeviceRequest deviceRequest = DeviceRequest(
      user: DeviceRequestUser(token: token, geoPoint: geoPoint),
      device: widget.deviceRequestDevice,
    );
    try {
      await sendDeviceRequest(deviceRequest);
      onSucess(context, deviceRequest);
    } catch (e) {
      print("$e while sending request");
      final errorSnackbar = SnackBar(
        content: Text(e.toString()),
        action: SnackBarAction(
          label: "Retry",
          onPressed: () => onSubmit(context, false),
        ),
      );
      Scaffold.of(context).showSnackBar(errorSnackbar);
    }
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
                iconData: getDeviceIcon(widget.firebaseDevice.type),
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
