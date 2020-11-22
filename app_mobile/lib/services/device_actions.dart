import 'package:firebase_auth/firebase_auth.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/models/devices/index.dart';
import 'package:houseflow/services/device.dart';
import 'package:houseflow/services/misc.dart';
import 'package:quick_actions/quick_actions.dart';

abstract class DeviceActions {
  static String removeActionPrefix(String sentence) {
    return sentence.replaceFirst(new RegExp('action_'), '');
  }

  static Future _actionsCallback(String shortcut, User currentUser) async {
    print("Recieved action from shortcut $shortcut");
    final DeviceRequestActions deviceRequestActions =
        DeviceRequestActions.values.firstWhere(
            (action) => action.stringify() == removeActionPrefix(shortcut));
    if (deviceRequestActions == null) {
      throw new Exception('Unexpected shortcut $shortcut');
    }

    final geoPoint = await getCurrentGeoPoint();

    final DeviceRequest deviceRequest = DeviceRequest(
        device: DeviceRequestDevice(
            action:
                // temporary solution, but currently every device accepts id: 1
                DeviceRequestAction(name: deviceRequestActions, id: 1)),
        user: DeviceRequestUser(
          geoPoint: geoPoint,
          token: await currentUser.getIdToken(),
        ));

    await sendDeviceRequest(deviceRequest);
  }

  static void initialize(User currentUser) {
    final QuickActions quickActions = QuickActions();
    quickActions
        .initialize((shortcut) => _actionsCallback(shortcut, currentUser));

    quickActions.setShortcutItems(<ShortcutItem>[
      const ShortcutItem(
          type: 'action_open_gate',
          localizedTitle: 'Open gate',
          icon: 'device_gate'),
      const ShortcutItem(
        type: 'action_mix_water',
        localizedTitle: 'Mix water',
        icon: 'device_watermixer',
      ),
      const ShortcutItem(
          type: 'action_open_garage',
          localizedTitle: 'Open garage',
          icon: 'device_garage'),
      const ShortcutItem(
          type: 'action_switch_lights',
          localizedTitle: 'Turn on lights',
          icon: 'device_lights'),
    ]);
  }
}
