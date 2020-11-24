enum DeviceActionTypes { OpenGate, OpenGarage, MixWater, SwitchLights }

extension DeviceActionsExtension on DeviceActionTypes {
  String stringify() {
    switch (this) {
      case DeviceActionTypes.MixWater:
        return 'mix_water';
      case DeviceActionTypes.OpenGate:
        return 'open_gate';
      case DeviceActionTypes.OpenGarage:
        return 'open_garage';
      case DeviceActionTypes.SwitchLights:
        return 'switch_lights';
      default:
        return null;
    }
  }
}

abstract class DeviceData {}
