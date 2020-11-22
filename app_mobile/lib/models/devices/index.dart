enum DeviceRequestActions { OpenGate, OpenGarage, MixWater, SwitchLights }

extension DeviceActionsExtension on DeviceRequestActions {
  String stringify() {
    switch (this) {
      case DeviceRequestActions.MixWater:
        return 'mix_water';
      case DeviceRequestActions.OpenGate:
        return 'open_garage';
      case DeviceRequestActions.OpenGarage:
        return 'open_garage';
      case DeviceRequestActions.SwitchLights:
        return 'switch_lights';
      default:
        return null;
    }
  }
}

abstract class DeviceData {}
