use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrightnessAbsolute {
    /// Brightness percentage.
    pub brightness: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum BrightnessRelative {
    /// Brightness percentage to change.
    Percent { brightness_relative_percent: i8 },
    /// Ambiguous amount to change the brightness, between -5 and +5.
    Weight { brightness_relative_weight: i8 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorAbsolute {
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub name: Option<String>,
    #[serde(flatten)]
    pub value: ColorValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ColorValue {
    Temperature {
        temperature: u16,
    },
    Rgb {
        #[serde(rename = "spectrumRGB")]
        spectrum_rgb: u32,
    },
    Hsv {
        #[serde(rename = "spectrumHSV")]
        spectrum_hsv: Hsv,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hsv {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnOff {
    pub on: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenClose {
    pub open_percent: u8,
}
