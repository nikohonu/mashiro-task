use std::fmt::{Debug, Display};

use regex::Regex;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Repeater {
    Absolute,
    Relative,
    Smart,
}

impl Display for Repeater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Repeater::Absolute => write!(f, "+"),
            Repeater::Relative => write!(f, ".+"),
            Repeater::Smart => write!(f, "++"),
        }
    }
}

impl Repeater {
    pub fn from_string(string: &str) -> Result<Self, &'static str> {
        match string {
            "+" => Ok(Repeater::Absolute),
            ".+" => Ok(Repeater::Relative),
            "++" => Ok(Repeater::Smart),
            _ => Err("Invalid repeater"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Yearly => write!(f, "y"),
            Unit::Monthly => write!(f, "m"),
            Unit::Weekly => write!(f, "w"),
            Unit::Daily => write!(f, "d"),
            Unit::Hourly => write!(f, "h"),
        }
    }
}

impl Unit {
    pub fn from_string(string: &str) -> Result<Self, &'static str> {
        match string {
            "y" => Ok(Unit::Yearly),
            "m" => Ok(Unit::Monthly),
            "w" => Ok(Unit::Weekly),
            "d" => Ok(Unit::Daily),
            "h" => Ok(Unit::Hourly),
            _ => Err("Invalid unit"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Recurrence {
    pub repeater: Repeater,
    pub value: u64,
    pub unit: Unit,
}

impl Recurrence {
    pub fn from_string(string: &str) -> Result<Self, &'static str> {
        let re = Regex::new(r"^(?<repeater>\+\+|\.\+|\+)(?<value>\d+)(?<unit>[ymwdh])").unwrap();
        if let Some(caps) = re.captures(string) {
            let repeater = Repeater::from_string(&caps["repeater"]);
            let value = caps["value"].parse::<u64>();
            let unit = Unit::from_string(&caps["unit"]);
            if repeater.is_ok() && value.is_ok() && unit.is_ok() {
                return Ok(Recurrence {
                    repeater: repeater.unwrap(),
                    value: value.unwrap(),
                    unit: unit.unwrap(),
                });
            }
        }
        Err("Can't parse this string")
    }
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.repeater, self.value, self.unit)
    }
}

impl Serialize for Recurrence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Recurrence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer).unwrap();
        match Recurrence::from_string(s) {
            Ok(recurrence) => Ok(recurrence),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(s),
                &"++4w | .+6m | +3d",
            )),
        }
    }
}
