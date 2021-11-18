use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[non_exhaustive]
pub enum Type {
    #[serde(rename = "action.devices.types.AC_UNIT")]
    AcUnit,
    #[serde(rename = "action.devices.types.AIRCOOLER")]
    Aircooler,
    #[serde(rename = "action.devices.types.AIRFRESHENER")]
    Airfreshener,
    #[serde(rename = "action.devices.types.AIRPURIFIER")]
    Airpurifier,
    #[serde(rename = "action.devices.types.AUDIO_VIDEO_RECEIVER")]
    AudioVideoReceiver,
    #[serde(rename = "action.devices.types.AWNING")]
    Awning,
    #[serde(rename = "action.devices.types.BATHTUB")]
    Bathtub,
    #[serde(rename = "action.devices.types.BED")]
    Bed,
    #[serde(rename = "action.devices.types.BLENDER")]
    Blender,
    #[serde(rename = "action.devices.types.BLINDS")]
    Blinds,
    #[serde(rename = "action.devices.types.BOILER")]
    Boiler,
    #[serde(rename = "action.devices.types.CAMERA")]
    Camera,
    #[serde(rename = "action.devices.types.CARBON_MONOXIDE_DETECTOR")]
    CarbonMonoxideDetector,
    #[serde(rename = "action.devices.types.CHARGER")]
    Charger,
    #[serde(rename = "action.devices.types.CLOSET")]
    Closet,
    #[serde(rename = "action.devices.types.COFFEE_MAKER")]
    CoffeeMaker,
    #[serde(rename = "action.devices.types.COOKTOP")]
    Cooktop,
    #[serde(rename = "action.devices.types.CURTAIN")]
    Curtain,
    #[serde(rename = "action.devices.types.DEHUMIDIFIER")]
    Dehumidifier,
    #[serde(rename = "action.devices.types.DEHYDRATOR")]
    Dehydrator,
    #[serde(rename = "action.devices.types.DISHWASHER")]
    Dishwasher,
    #[serde(rename = "action.devices.types.DOOR")]
    Door,
    #[serde(rename = "action.devices.types.DOORBELL")]
    Doorbell,
    #[serde(rename = "action.devices.types.DRAWER")]
    Drawer,
    #[serde(rename = "action.devices.types.DRYER")]
    Dryer,
    #[serde(rename = "action.devices.types.FAN")]
    Fan,
    #[serde(rename = "action.devices.types.FAUCET")]
    Faucet,
    #[serde(rename = "action.devices.types.FIREPLACE")]
    Fireplace,
    #[serde(rename = "action.devices.types.FREEZER")]
    Freezer,
    #[serde(rename = "action.devices.types.FRYER")]
    Fryer,
    #[serde(rename = "action.devices.types.GARAGE")]
    Garage,
    #[serde(rename = "action.devices.types.GATE")]
    Gate,
    #[serde(rename = "action.devices.types.GRILL")]
    Grill,
    #[serde(rename = "action.devices.types.HEATER")]
    Heater,
    #[serde(rename = "action.devices.types.HOOD")]
    Hood,
    #[serde(rename = "action.devices.types.HUMIDIFIER")]
    Humidifier,
    #[serde(rename = "action.devices.types.KETTLE")]
    Kettle,
    #[serde(rename = "action.devices.types.LIGHT")]
    Light,
    #[serde(rename = "action.devices.types.LOCK")]
    Lock,
    #[serde(rename = "action.devices.types.MICROWAVE")]
    Microwave,
    #[serde(rename = "action.devices.types.MOP")]
    Mop,
    #[serde(rename = "action.devices.types.MOWER")]
    Mower,
    #[serde(rename = "action.devices.types.MULTICOOKER")]
    Multicooker,
    #[serde(rename = "action.devices.types.NETWORK")]
    Network,
    #[serde(rename = "action.devices.types.OUTLET")]
    Outlet,
    #[serde(rename = "action.devices.types.OVEN")]
    Oven,
    #[serde(rename = "action.devices.types.PERGOLA")]
    Pergola,
    #[serde(rename = "action.devices.types.PETFEEDER")]
    Petfeeder,
    #[serde(rename = "action.devices.types.PRESSURECOOKER")]
    Pressurecooker,
    #[serde(rename = "action.devices.types.RADIATOR")]
    Radiator,
    #[serde(rename = "action.devices.types.REFRIGERATOR")]
    Refrigerator,
    #[serde(rename = "action.devices.types.REMOTECONTROL")]
    Remotecontrol,
    #[serde(rename = "action.devices.types.ROUTER")]
    Router,
    #[serde(rename = "action.devices.types.SCENE")]
    Scene,
    #[serde(rename = "action.devices.types.SECURITYSYSTEM")]
    Securitysystem,
    #[serde(rename = "action.devices.types.SENSOR")]
    Sensor,
    #[serde(rename = "action.devices.types.SETTOP")]
    Settop,
    #[serde(rename = "action.devices.types.SHOWER")]
    Shower,
    #[serde(rename = "action.devices.types.SHUTTER")]
    Shutter,
    #[serde(rename = "action.devices.types.SMOKE_DETECTOR")]
    SmokeDetector,
    #[serde(rename = "action.devices.types.SOUNDBAR")]
    Soundbar,
    #[serde(rename = "action.devices.types.SOUSVIDE")]
    Sousvide,
    #[serde(rename = "action.devices.types.SPEAKER")]
    Speaker,
    #[serde(rename = "action.devices.types.SPRINKLER")]
    Sprinkler,
    #[serde(rename = "action.devices.types.STANDMIXER")]
    Standmixer,
    #[serde(rename = "action.devices.types.STREAMING_BOX")]
    StreamingBox,
    #[serde(rename = "action.devices.types.STREAMING_SOUNDBAR")]
    StreamingSoundbar,
    #[serde(rename = "action.devices.types.STREAMING_STICK")]
    StreamingStick,
    #[serde(rename = "action.devices.types.SWITCH")]
    Switch,
    #[serde(rename = "action.devices.types.THERMOSTAT")]
    Thermostat,
    #[serde(rename = "action.devices.types.TV")]
    Tv,
    #[serde(rename = "action.devices.types.VACUUM")]
    Vacuum,
    #[serde(rename = "action.devices.types.VALVE")]
    Valve,
    #[serde(rename = "action.devices.types.WASHER")]
    Washer,
    #[serde(rename = "action.devices.types.WATERHEATER")]
    Waterheater,
    #[serde(rename = "action.devices.types.WATERPURIFIER")]
    Waterpurifier,
    #[serde(rename = "action.devices.types.WATERSOFTENER")]
    Watersoftener,
    #[serde(rename = "action.devices.types.WINDOW")]
    Window,
    #[serde(rename = "action.devices.types.YOGURTMAKER")]
    Yogurtmaker,
}
