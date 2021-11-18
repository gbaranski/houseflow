use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[non_exhaustive]
pub enum Trait {
    #[serde(rename = "action.devices.traits.AppSelector")]
    AppSelector,
    #[serde(rename = "action.devices.traits.ArmDisarm")]
    ArmDisarm,
    #[serde(rename = "action.devices.traits.Brightness")]
    Brightness,
    #[serde(rename = "action.devices.traits.CameraStream")]
    CameraStream,
    #[serde(rename = "action.devices.traits.Channel")]
    Channel,
    #[serde(rename = "action.devices.traits.ColorSetting")]
    ColorSetting,
    #[serde(rename = "action.devices.traits.Cook")]
    Cook,
    #[serde(rename = "action.devices.traits.Dispense")]
    Dispense,
    #[serde(rename = "action.devices.traits.Dock")]
    Dock,
    #[serde(rename = "action.devices.traits.EnergyStorage")]
    EnergyStorage,
    #[serde(rename = "action.devices.traits.FanSpeed")]
    FanSpeed,
    #[serde(rename = "action.devices.traits.Fill")]
    Fill,
    #[serde(rename = "action.devices.traits.HumiditySetting")]
    HumiditySetting,
    #[serde(rename = "action.devices.traits.InputSelector")]
    InputSelector,
    #[serde(rename = "action.devices.traits.LightEffects")]
    LightEffects,
    #[serde(rename = "action.devices.traits.Locator")]
    Locator,
    #[serde(rename = "action.devices.traits.LockUnlock")]
    LockUnlock,
    #[serde(rename = "action.devices.traits.MediaState")]
    MediaState,
    #[serde(rename = "action.devices.traits.Modes")]
    Modes,
    #[serde(rename = "action.devices.traits.NetworkControl")]
    NetworkControl,
    #[serde(rename = "action.devices.traits.ObjectDetection")]
    ObjectDetection,
    #[serde(rename = "action.devices.traits.OnOff")]
    OnOff,
    #[serde(rename = "action.devices.traits.OpenClose")]
    OpenClose,
    #[serde(rename = "action.devices.traits.Reboot")]
    Reboot,
    #[serde(rename = "action.devices.traits.Rotation")]
    Rotation,
    #[serde(rename = "action.devices.traits.RunCycle")]
    RunCycle,
    #[serde(rename = "action.devices.traits.Scene")]
    Scene,
    #[serde(rename = "action.devices.traits.SensorState")]
    SensorState,
    #[serde(rename = "action.devices.traits.SoftwareUpdate")]
    SoftwareUpdate,
    #[serde(rename = "action.devices.traits.StartStop")]
    StartStop,
    #[serde(rename = "action.devices.traits.StatusReport")]
    StatusReport,
    #[serde(rename = "action.devices.traits.TemperatureControl")]
    TemperatureControl,
    #[serde(rename = "action.devices.traits.TemperatureSetting")]
    TemperatureSetting,
    #[serde(rename = "action.devices.traits.Timer")]
    Timer,
    #[serde(rename = "action.devices.traits.Toggles")]
    Toggles,
    #[serde(rename = "action.devices.traits.TransportControl")]
    TransportControl,
    #[serde(rename = "action.devices.traits.Volume")]
    Volume,
}
