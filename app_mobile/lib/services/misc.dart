import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';
import 'package:houseflow/models/device.dart';

Widget locationAgreementAlertDialog(BuildContext context) => AlertDialog(
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

Future<GeoPoint> getCurrentGeoPoint([BuildContext context]) async {
  try {
    Position position = await Geolocator.getLastKnownPosition();
    if (position == null)
      position = await Geolocator.getCurrentPosition(
          desiredAccuracy: LocationAccuracy.high);
    return GeoPoint(latitude: position.latitude, longitude: position.longitude);
  } catch (e) {
    print(e);
    if (context != null)
      showDialog(
          context: context,
          builder: (context) => locationAgreementAlertDialog(context));
    return null;
  }
}
