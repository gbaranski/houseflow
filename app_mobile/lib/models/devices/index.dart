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

  String stringify2() {
    switch (this) {
      case DeviceActionTypes.MixWater:
        return 'Mix water';
      case DeviceActionTypes.OpenGate:
        return 'Open gate';
      case DeviceActionTypes.OpenGarage:
        return 'Open garage';
      case DeviceActionTypes.SwitchLights:
        return 'Switch lights';
      default:
        return null;
    }
  }
}

abstract class DeviceData {}
