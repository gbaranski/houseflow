import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/screens/single_device/single_device.dart';
import 'package:houseflow/utils/misc.dart';

class DeviceCard extends StatelessWidget {
  final Color color;
  final FirebaseDevice firebaseDevice;
  final Function() onValidTap;
  final IconData iconData;

  const DeviceCard(
      {Key key,
      @required this.color,
      @required this.firebaseDevice,
      @required this.onValidTap,
      @required this.iconData})
      : super(key: key);

  Widget verifyDeviceStatus({@required Widget child}) {
    if (firebaseDevice.status) {
      return child;
    } else {
      return Banner(
        location: BannerLocation.topStart,
        message: "OFFLINE",
        child: child,
      );
    }
  }

  void showDeviceOfflineSnackbar(BuildContext context) {
    const snackbar = SnackBar(
      content: Text("Device is offline!"),
      duration: Duration(milliseconds: 600),
    );
    Scaffold.of(context).showSnackBar(snackbar);
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      color: color,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(20.0)),
      child: InkWell(
        onLongPress: () => Navigator.push(
            context,
            MaterialPageRoute(
                settings: const RouteSettings(name: 'Device info'),
                builder: (context) =>
                    SingleDevice(firebaseDevice: firebaseDevice))),
        splashColor: Colors.white.withAlpha(100),
        onTap: firebaseDevice.status
            ? onValidTap
            : () => showDeviceOfflineSnackbar(context),
        child: ClipRRect(
          child: verifyDeviceStatus(
              child: Container(
            margin: const EdgeInsets.all(10),
            child: LayoutBuilder(
              builder: (context, constraint) => Column(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  Icon(iconData,
                      color: Colors.white.withAlpha(180),
                      size: constraint.biggest.width / 2),
                  Text(
                    upperFirstCharacter(firebaseDevice.type),
                    style: TextStyle(
                        color: Colors.white.withAlpha(190),
                        fontSize: constraint.biggest.width / 8,
                        fontWeight: FontWeight.w100),
                  ),
                ],
              ),
            ),
          )),
        ),
      ),
    );
  }
}
