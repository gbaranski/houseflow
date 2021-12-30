use homie_controller::Datatype;
use homie_controller::Device;
use homie_controller::Node;
use homie_controller::Property;
use serde_json::Number;
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

fn cap<N: Copy + PartialOrd>(value: N, min: N, max: N) -> N {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
