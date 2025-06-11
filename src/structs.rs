use std::{fmt::Display, str::FromStr};

use crate::{enums::FlightRules, errors::FsdMessageParseError, Level};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransponderCode(u16);
impl TryFrom<u16> for TransponderCode {
    type Error = FsdMessageParseError;
    fn try_from(mut code: u16) -> Result<Self, Self::Error> {
        let mut digits = [0; 4];
        digits[0] = code / 1000;
        code -= digits[0] * 1000;
        digits[1] = code / 100;
        code -= digits[1] * 100;
        digits[2] = code / 10;
        code -= digits[2] * 10;
        digits[3] = code;

        if digits.into_iter().any(|x| x > 7) {
            Err(FsdMessageParseError::InvalidTransponderCode(format!(
                "{:04}",
                code
            )))
        } else {
            let code = digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];
            Ok(TransponderCode(code))
        }
    }
}
impl TransponderCode {
    pub fn try_from_bcd_format(
        bcd: impl AsRef<str>,
    ) -> Result<TransponderCode, FsdMessageParseError> {
        let bcd = bcd.as_ref();
        let num = bcd
            .parse::<u32>()
            .map_err(|_| FsdMessageParseError::InvalidTransponderCode(bcd.to_owned()))?;
        let hex_value = format!("{:X}", num);
        let result = hex_value
            .parse::<u16>()
            .map_err(|_| FsdMessageParseError::InvalidTransponderCode(bcd.to_owned()))?;
        Self::try_from(result)
    }
    pub fn as_bcd_format(&self) -> u32 {
        let as_dec_str = self.to_string();
        u32::from_str_radix(&as_dec_str, 16).unwrap()
    }
}

impl FromStr for TransponderCode {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: u16 = s
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidTransponderCode(s.to_string()))?;
        code.try_into()
    }
}
impl Display for TransponderCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

/// Represents a VHF, airband radio frequency from 118.000 MHz to 137.000 MHz.
///
/// Stored internally as the left part and the right part. For example, 118.3MHz is `RadioFrequency(118, 300)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadioFrequency(pub(crate) u16, pub(crate) u16);
impl RadioFrequency {
    /// Creates a new [`RadioFrequency`] from two parts
    ///
    /// # Example
    /// ```
    /// use fsd_interface::RadioFrequency;
    /// let freq = RadioFrequency::new(118, 300).unwrap();
    /// assert_eq!((118, 300), freq.frequency());
    /// ```
    pub fn new(left: u16, right: u16) -> Result<RadioFrequency, FsdMessageParseError> {
        if !((118..=137).contains(&left)
            || left == 199 && right == 998
            || left == 149 && right == 999)
        {
            return Err(FsdMessageParseError::InvalidFrequency(format!(
                "{}.{:03}",
                left, right
            )));
        }
        Ok(RadioFrequency(left, right))
    }

    pub fn frequency(&self) -> (u16, u16) {
        (self.0, self.1)
    }

    /// Returns the frequency in the form XXX.YYY
    ///
    /// # Example
    /// ```
    /// use fsd_interface::RadioFrequency;
    /// let freq = RadioFrequency::new(133, 175).unwrap();
    /// let human_readable = freq.to_human_readable_string();
    /// assert_eq!(human_readable, String::from("133.175"));
    /// ```
    pub fn to_human_readable_string(&self) -> String {
        format!("{}.{:03}", self.0, self.1)
    }
    pub fn try_from_human_readable_string(
        input: impl AsRef<str>,
    ) -> Result<RadioFrequency, FsdMessageParseError> {
        let input = input.as_ref();
        let mut split = input.split('.');

        let left = split
            .next()
            .ok_or_else(|| FsdMessageParseError::InvalidFrequency(input.to_string()))?;
        let left: u16 = left
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidFrequency(input.to_string()))?;
        let right = split
            .next()
            .ok_or_else(|| FsdMessageParseError::InvalidFrequency(input.to_string()))?;
        let right: u16 = right
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidFrequency(input.to_string()))?;
        RadioFrequency::new(left, right)
    }
}

impl FromStr for RadioFrequency {
    type Err = FsdMessageParseError;
    fn from_str(short_form: &str) -> Result<Self, Self::Err> {
        if short_form.len() != 5 {
            return Err(FsdMessageParseError::InvalidFrequency(
                short_form.to_string(),
            ));
        }
        let left: u16 = short_form[0..2]
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidFrequency(short_form.to_string()))?;
        let right: u16 = short_form[2..]
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidFrequency(short_form.to_string()))?;
        RadioFrequency::new(left + 100, right)
    }
}

impl Display for RadioFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:03}", self.0 - 100, self.1)
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlaneInfo {
    pub equipment: Option<String>,
    pub airline: Option<String>,
    pub livery: Option<String>,
}
impl From<&[&str]> for PlaneInfo {
    fn from(value: &[&str]) -> Self {
        let mut plane_info = PlaneInfo::default();

        if value.is_empty() {
            return plane_info;
        }

        for entry in value {
            let mut split = entry.split('=');
            let k = match split.next() {
                Some(k) => k,
                None => continue,
            };

            let v = match split.next() {
                Some(v) => v.to_string(),
                None => continue,
            };

            match k.to_uppercase().as_str() {
                "EQUIPMENT" => plane_info.equipment = Some(v),
                "AIRLINE" => plane_info.airline = Some(v),
                "LIVERY" => plane_info.livery = Some(v),
                _ => {}
            }
        }
        plane_info
    }
}
impl Display for PlaneInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut need_delimiter = false;
        if let Some(ref equipment) = self.equipment {
            write!(f, "EQUIPMENT={}", equipment)?;
            need_delimiter = true;
        }
        if let Some(ref airline) = self.airline {
            if need_delimiter {
                write!(f, ":")?;
            }
            write!(f, "AIRLINE={}", airline)?;
            need_delimiter = true;
        }
        if let Some(ref livery) = self.livery {
            if need_delimiter {
                write!(f, ":")?;
            }
            write!(f, "LIVERY={}", livery)?;
        }
        Ok(())
    }
}

impl PlaneInfo {
    pub fn new(
        equipment: Option<impl Into<String>>,
        airline: Option<impl Into<String>>,
        livery: Option<impl Into<String>>,
    ) -> PlaneInfo {
        let equipment = equipment.map(|x| x.into());
        let airline = airline.map(|x| x.into());
        let livery = livery.map(|x| x.into());
        PlaneInfo {
            equipment,
            airline,
            livery,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlightPlan {
    pub flight_rules: FlightRules,
    pub ac_type: String,
    pub filed_tas: u16,
    pub origin: String,
    pub etd: u16,
    pub atd: u16,
    pub cruise_level: Level,
    pub destination: String,
    pub hours_enroute: u8,
    pub mins_enroute: u8,
    pub hours_fuel: u8,
    pub mins_fuel: u8,
    pub alternate: String,
    pub remarks: String,
    pub route: String,
}

impl Display for FlightPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.flight_rules,
            self.ac_type,
            self.filed_tas,
            self.origin,
            self.etd,
            self.atd,
            self.cruise_level,
            self.destination,
            self.hours_enroute,
            self.mins_enroute,
            self.hours_fuel,
            self.mins_fuel,
            self.alternate,
            self.remarks,
            self.route
        )
    }
}

impl TryFrom<&[&str]> for FlightPlan {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        if fields.len() != 15 {
            return Err(FsdMessageParseError::InvalidFieldCount(15, fields.len()));
        }

        let filed_tas = if fields[2].is_empty() {
            0
        } else {
            fields[2]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidSpeed(fields[2].to_string()))?
        };
        let etd = if fields[4].is_empty() {
            0
        } else {
            fields[4]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidTime(fields[4].to_string()))?
        };
        let atd = if fields[5].is_empty() {
            0
        } else {
            fields[5]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidTime(fields[5].to_string()))?
        };
        let hours_enroute = if fields[8].is_empty() {
            0
        } else {
            fields[8]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidTime(fields[8].to_string()))?
        };
        let hours_fuel = if fields[10].is_empty() {
            0
        } else {
            fields[10]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidTime(fields[10].to_string()))?
        };
        let mins_enroute = if fields[9].is_empty() {
            0
        } else {
            let mins = fields[9]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidTime(fields[9].to_string()))?;
            if mins > 59 {
                return Err(FsdMessageParseError::InvalidMinute(fields[9].to_string()));
            }
            mins
        };
        let mins_fuel = if fields[11].is_empty() {
            0
        } else {
            let mins = fields[11]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidTime(fields[11].to_string()))?;
            if mins > 59 {
                return Err(FsdMessageParseError::InvalidMinute(fields[11].to_string()));
            }
            mins
        };

        Ok(FlightPlan::new(
            fields[0].parse()?,
            fields[1],
            filed_tas,
            fields[3],
            etd,
            atd,
            fields[6].parse()?,
            fields[7],
            hours_enroute,
            mins_enroute,
            hours_fuel,
            mins_fuel,
            fields[12],
            fields[13],
            fields[14],
        ))
    }
}

impl FlightPlan {
    pub fn new(
        flight_rules: FlightRules,
        ac_type: impl AsRef<str>,
        filed_tas: u16,
        origin: impl AsRef<str>,
        etd: u16,
        atd: u16,
        cruise_level: Level,
        destination: impl AsRef<str>,
        hours_enroute: u8,
        mins_enroute: u8,
        hours_fuel: u8,
        mins_fuel: u8,
        alternate: impl AsRef<str>,
        remarks: impl Into<String>,
        route: impl AsRef<str>,
    ) -> FlightPlan {
        FlightPlan {
            flight_rules,
            ac_type: ac_type.as_ref().to_uppercase(),
            filed_tas,
            origin: origin.as_ref().to_uppercase(),
            etd,
            atd,
            cruise_level,
            destination: destination.as_ref().to_uppercase(),
            hours_enroute,
            mins_enroute,
            hours_fuel,
            mins_fuel,
            alternate: alternate.as_ref().to_uppercase(),
            remarks: remarks.into(),
            route: route.as_ref().to_uppercase(),
        }
    }
}
