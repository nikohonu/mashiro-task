use anyhow::{anyhow, Result};
use std::fmt::{Debug, Display};

use regex::Regex;

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
    pub fn from_string(string: &str) -> Result<Self> {
        match string {
            "+" => Ok(Repeater::Absolute),
            ".+" => Ok(Repeater::Relative),
            "++" => Ok(Repeater::Smart),
            _ => Err(anyhow!("Invalid repeater")),
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
    pub fn from_string(string: &str) -> Result<Self> {
        match string {
            "y" => Ok(Unit::Yearly),
            "m" => Ok(Unit::Monthly),
            "w" => Ok(Unit::Weekly),
            "d" => Ok(Unit::Daily),
            "h" => Ok(Unit::Hourly),
            _ => Err(anyhow!("Invalid unit")),
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
    pub fn from_string(string: &str) -> Result<Self> {
        let re = Regex::new(r"^(?<repeater>\+\+|\.\+|\+)(?<value>\d+)(?<unit>[ymwdh])")?;
        let caps = re
            .captures(string)
            .expect(format!("Can't captures: \"{}\"", string).as_str());
        let repeater = Repeater::from_string(&caps["repeater"])?;
        let value = caps["value"].parse::<u64>()?;
        let unit = Unit::from_string(&caps["unit"])?;
        return Ok(Recurrence {
            repeater,
            value,
            unit,
        });
    }
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.repeater, self.value, self.unit)
    }
}
