use google_smart_home::device::commands::ColorAbsolute;
use google_smart_home::device::commands::ColorValue;
use homie_controller::ColorFormat;
use homie_controller::ColorHsv;
use homie_controller::ColorRgb;
use homie_controller::Datatype;
use homie_controller::Device;
use homie_controller::Node;
use homie_controller::Property;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use std::collections::HashMap;
use std::ops::RangeInclusive;

/// Given an ID of the form `"device_id/node_id"`, looks up the corresponding Homie node (if any).
pub fn get_homie_device_by_id<'a>(
    devices: &'a HashMap<String, Device>,
    id: &str,
) -> Option<(&'a Device, &'a Node)> {
    let id_parts: Vec<_> = id.split('/').collect();
    if let [device_id, node_id] = id_parts.as_slice() {
        if let Some(device) = devices.get(*device_id) {
            if let Some(node) = device.nodes.get(*node_id) {
                return Some((device, node));
            }
        }
    }

    None
}

/// Scales the value of the given property to a percentage.
pub fn property_value_to_percentage(property: &Property) -> Option<u8> {
    match property.datatype? {
        Datatype::Integer => {
            let value: i64 = property.value().ok()?;
            let range: RangeInclusive<i64> = property.range().ok()?;
            let percentage = (value - range.start()) * 100 / (range.end() - range.start());
            let percentage = cap(percentage, 0, 100);
            Some(percentage as u8)
        }
        Datatype::Float => {
            let value: f64 = property.value().ok()?;
            let range: RangeInclusive<f64> = property.range().ok()?;
            let percentage = (value - range.start()) * 100.0 / (range.end() - range.start());
            let percentage = cap(percentage, 0.0, 100.0);
            Some(percentage as u8)
        }
        _ => None,
    }
}

/// Converts a percentage to the appropriately scaled property value of the given property, if it has
/// a range specified.
pub fn percentage_to_property_value(property: &Property, percentage: u8) -> Option<String> {
    match property.datatype? {
        Datatype::Integer => {
            let range: RangeInclusive<i64> = property.range().ok()?;
            let value = range.start() + percentage as i64 * (range.end() - range.start()) / 100;
            Some(format!("{}", value))
        }
        Datatype::Float => {
            let range: RangeInclusive<f64> = property.range().ok()?;
            let value = range.start() + percentage as f64 * (range.end() - range.start()) / 100.0;
            Some(format!("{}", value))
        }
        _ => None,
    }
}

/// Converts the property value to a JSON number if it is an appropriate type.
pub fn property_value_to_number(property: &Property) -> Option<Number> {
    match property.datatype? {
        Datatype::Integer => {
            let value: i64 = property.value().ok()?;
            Some(value.into())
        }
        Datatype::Float => {
            let value = property.value().ok()?;
            Number::from_f64(value)
        }
        _ => None,
    }
}

/// Converts the value of the given property to a Google Home JSON color value, if it is the
/// appropriate type.
pub fn property_value_to_color(property: &Property) -> Option<Map<String, Value>> {
    let color_format = property.color_format().ok()?;
    let mut color_value = Map::new();
    match color_format {
        ColorFormat::Rgb => {
            let rgb: ColorRgb = property.value().ok()?;
            let rgb_int = ((rgb.r as u32) << 16) + ((rgb.g as u32) << 8) + (rgb.b as u32);
            color_value.insert("spectrumRgb".to_string(), Value::Number(rgb_int.into()))
        }
        ColorFormat::Hsv => {
            let hsv: ColorHsv = property.value().ok()?;
            let mut hsv_map = Map::new();
            hsv_map.insert("hue".to_string(), Value::Number(hsv.h.into()));
            hsv_map.insert(
                "saturation".to_string(),
                Value::Number(Number::from_f64(hsv.s as f64 / 100.0)?),
            );
            hsv_map.insert(
                "value".to_string(),
                Value::Number(Number::from_f64(hsv.v as f64 / 100.0)?),
            );
            color_value.insert("spectrumHsv".to_string(), Value::Object(hsv_map))
        }
    };
    Some(color_value)
}

/// Converts a Google Home `ColorAbsolute` command to the appropriate value to set on the given
/// Homie property, if it is the appropriate format.
pub fn color_absolute_to_property_value(
    property: &Property,
    color_absolute: &ColorAbsolute,
) -> Option<String> {
    let color_format = property.color_format().ok()?;
    match color_format {
        ColorFormat::Rgb => {
            if let ColorValue::Rgb { spectrum_rgb } = color_absolute.color.value {
                let rgb = ColorRgb::new(
                    (spectrum_rgb >> 16) as u8,
                    (spectrum_rgb >> 8) as u8,
                    spectrum_rgb as u8,
                );
                return Some(rgb.to_string());
            }
        }
        ColorFormat::Hsv => {
            if let ColorValue::Hsv { spectrum_hsv } = &color_absolute.color.value {
                let hsv = ColorHsv::new(
                    spectrum_hsv.hue as u16,
                    (spectrum_hsv.saturation * 100.0) as u8,
                    (spectrum_hsv.value * 100.0) as u8,
                );
                return Some(hsv.to_string());
            }
        }
    }
    None
}

fn cap<N: Copy + PartialOrd>(value: N, min: N, max: N) -> N {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use google_smart_home::device::commands::{Color, Hsv};
    use serde_json::json;

    use super::*;

    #[test]
    fn percentage_integer() {
        let property = Property {
            id: "brightness".to_string(),
            name: Some("Brightness".to_string()),
            datatype: Some(Datatype::Integer),
            settable: true,
            retained: true,
            unit: None,
            format: Some("10:20".to_string()),
            value: Some("13".to_string()),
        };

        assert_eq!(property_value_to_percentage(&property), Some(30));
        assert_eq!(
            percentage_to_property_value(&property, 70),
            Some("17".to_string())
        );
    }

    #[test]
    fn percentage_float() {
        let property = Property {
            id: "brightness".to_string(),
            name: Some("Brightness".to_string()),
            datatype: Some(Datatype::Float),
            settable: true,
            retained: true,
            unit: None,
            format: Some("1.0:2.0".to_string()),
            value: Some("1.3".to_string()),
        };

        assert_eq!(property_value_to_percentage(&property), Some(30));
        assert_eq!(
            percentage_to_property_value(&property, 70),
            Some("1.7".to_string())
        );
    }

    #[test]
    fn number_integer() {
        let property = Property {
            id: "number".to_string(),
            name: Some("Number".to_string()),
            datatype: Some(Datatype::Integer),
            settable: true,
            retained: true,
            unit: None,
            format: None,
            value: Some("42".to_string()),
        };

        assert_eq!(
            property_value_to_number(&property).unwrap().as_u64(),
            Some(42)
        );
    }

    #[test]
    fn number_float() {
        let property = Property {
            id: "number".to_string(),
            name: Some("Number".to_string()),
            datatype: Some(Datatype::Float),
            settable: true,
            retained: true,
            unit: None,
            format: None,
            value: Some("42.2".to_string()),
        };

        assert_eq!(
            property_value_to_number(&property).unwrap().as_f64(),
            Some(42.2)
        );
    }

    #[test]
    fn color_rgb() {
        let property = Property {
            id: "color".to_string(),
            name: Some("Colour".to_string()),
            datatype: Some(Datatype::Color),
            settable: true,
            retained: true,
            unit: None,
            format: Some("rgb".to_string()),
            value: Some("17,34,51".to_string()),
        };

        assert_eq!(
            property_value_to_color(&property)
                .unwrap()
                .get("spectrumRgb"),
            Some(&Value::Number(0x112233.into()))
        );
        assert_eq!(
            color_absolute_to_property_value(
                &property,
                &ColorAbsolute {
                    color: Color {
                        name: None,
                        value: ColorValue::Rgb {
                            spectrum_rgb: 0x445566
                        }
                    }
                }
            ),
            Some("68,85,102".to_string())
        );
    }

    #[test]
    fn color_hsv() {
        let property = Property {
            id: "color".to_string(),
            name: Some("Colour".to_string()),
            datatype: Some(Datatype::Color),
            settable: true,
            retained: true,
            unit: None,
            format: Some("hsv".to_string()),
            value: Some("280,50,60".to_string()),
        };

        assert_eq!(
            property_value_to_color(&property)
                .unwrap()
                .get("spectrumHsv"),
            Some(&json!({"hue": 280, "saturation": 0.5, "value": 0.6}))
        );
        assert_eq!(
            color_absolute_to_property_value(
                &property,
                &ColorAbsolute {
                    color: Color {
                        name: None,
                        value: ColorValue::Hsv {
                            spectrum_hsv: Hsv {
                                hue: 290.0,
                                saturation: 0.2,
                                value: 0.3
                            }
                        }
                    }
                }
            ),
            Some("290,20,30".to_string())
        );
    }
}
