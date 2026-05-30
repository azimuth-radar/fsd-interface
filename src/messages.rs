//! Contains all the message types
//!
//!

use std::{fmt::Display, net::Ipv4Addr};

use chrono::NaiveDateTime;

use crate::{
    LandLineCommand, LandLineType, Level, ScratchPad,
    aircraft_config::AircraftConfig,
    enums::{
        AtcRating, AtcType, AtisLine, ClientCapability, ClientQueryType, ClientResponseType,
        PilotRating, ProtocolRevision, SharedStateType, SimulatorType, TransponderMode,
        VoiceCapability,
    },
    errors::{FsdError, FsdMessageParseError},
    structs::{FlightPlan, PlaneInfo, RadioFrequency, TransponderCode},
    util,
};

pub const SERVER_CALLSIGN: &str = "SERVER";
pub const ATC_TEXT_CHANNEL_FREQUENCY: RadioFrequency = RadioFrequency(149, 999);
pub const AIRCRAFT_HANDLER_RECIPIENT: &str = "@94835";

macro_rules! check_min_num_fields {
    ($fields: ident, $i: literal) => {
        if $fields.len() < $i {
            return Err(FsdMessageParseError::InvalidFieldCount($i, $fields.len()));
        }
    };
}

macro_rules! check_exact_num_fields {
    ($fields: ident, $i: literal) => {
        if $fields.len() != $i {
            return Err(FsdMessageParseError::InvalidFieldCount($i, $fields.len()));
        }
    };
}

/// Sent by an ATC client to register itself on the network after the initial handshake
#[derive(Debug, Clone)]
pub struct AtcRegisterMessage {
    pub from: String,
    pub to: String,
    pub real_name: String,
    pub cid: String,
    pub password: String,
    pub rating: AtcRating,
    pub protocol: ProtocolRevision,
}
//#AAEGPH_M_APP:SERVER:Caspian:newcert:test:4:9:1:0:55.95000:-3.37250:100

impl Display for AtcRegisterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#AA{}:{}:{}:{}:{}:{}:{}",
            self.from,
            self.to,
            self.real_name,
            self.cid,
            self.password,
            self.rating as u8,
            self.protocol as u8
        )
    }
}

impl TryFrom<&[&str]> for AtcRegisterMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 7);
        let first = &fields[0][3..];
        Ok(AtcRegisterMessage::new(
            first,
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5].parse()?,
            fields.get(6).unwrap_or(&"9").parse()?,
        ))
    }
}

impl AtcRegisterMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        real_name: impl Into<String>,
        cid: impl Into<String>,
        password: impl Into<String>,
        rating: AtcRating,
        protocol: ProtocolRevision,
    ) -> Self {
        AtcRegisterMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            real_name: real_name.into(),
            cid: cid.into(),
            password: password.into(),
            rating,
            protocol,
        }
    }
}

/// Sent by a pilot client to register itself on the network after the initial handshake
#[derive(Debug, Clone)]
pub struct PilotRegisterMessage {
    pub from: String,
    pub to: String,
    pub cid: String,
    pub password: String,
    pub rating: PilotRating,
    pub protocol: ProtocolRevision,
    pub simulator_type: SimulatorType,
    pub real_name: String,
}
//#APEZY38UB:SERVER:newcert::1:1:1
impl Display for PilotRegisterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#AP{}:{}:{}:{}:{}:{}:{}:{}",
            self.from,
            self.to,
            self.cid,
            self.password,
            self.rating as u8,
            self.protocol as u8,
            self.simulator_type as u8,
            self.real_name
        )
    }
}
impl TryFrom<&[&str]> for PilotRegisterMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 7);
        let first = &fields[0][3..];
        Ok(PilotRegisterMessage::new(
            first,
            fields[1],
            *fields.get(7).unwrap_or(&""),
            fields[2],
            fields[3],
            fields[4].parse()?,
            fields[5].parse()?,
            fields[6].into(),
        ))
    }
}
impl PilotRegisterMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        real_name: impl Into<String>,
        cid: impl Into<String>,
        password: impl Into<String>,
        rating: PilotRating,
        protocol: ProtocolRevision,
        simulator_type: SimulatorType,
    ) -> Self {
        PilotRegisterMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            simulator_type,
            cid: cid.into(),
            password: password.into(),
            rating,
            protocol,
            real_name: real_name.into(),
        }
    }
}

/// Sent by an ATC client before disconnecting
#[derive(Clone, Debug)]
pub struct AtcDeregisterMessage {
    pub from: String,
    pub cid: Option<String>,
}

impl Display for AtcDeregisterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cid) = &self.cid {
            write!(f, "#DA{}:{}", self.from, cid)
        } else {
            write!(f, "#DA{}", self.from)
        }
    }
}

impl TryFrom<&[&str]> for AtcDeregisterMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 1);
        let first = &fields[0][3..];
        let cid = fields.get(1).map(|s| s.to_string());
        Ok(AtcDeregisterMessage {
            from: first.to_uppercase(),
            cid,
        })
    }
}

impl AtcDeregisterMessage {
    pub fn new(from: impl AsRef<str>, cid: impl Into<String>) -> Self {
        AtcDeregisterMessage {
            from: from.as_ref().to_uppercase(),
            cid: Some(cid.into()),
        }
    }
}

/// Sent by a pilot client before disconnecting
#[derive(Clone, Debug)]
pub struct PilotDeregisterMessage {
    pub from: String,
    pub cid: Option<String>,
}

impl Display for PilotDeregisterMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cid) = &self.cid {
            write!(f, "#DP{}:{}", self.from, cid)
        } else {
            write!(f, "#DP{}", self.from)
        }
    }
}

impl TryFrom<&[&str]> for PilotDeregisterMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 1);
        let first = &fields[0][3..];
        let cid = fields.get(1).map(|s| s.to_string());
        Ok(PilotDeregisterMessage {
            from: first.to_uppercase(),
            cid,
        })
    }
}

impl PilotDeregisterMessage {
    pub fn new(from: impl AsRef<str>, cid: impl Into<String>) -> Self {
        PilotDeregisterMessage {
            from: from.as_ref().to_uppercase(),
            cid: Some(cid.into()),
        }
    }
}

/// Sent at regular intervals by an ATC client to update the server with its position
#[derive(Debug, Clone, PartialEq)]
pub struct AtcPositionUpdateMessage {
    pub callsign: String,
    pub frequencies: Vec<RadioFrequency>,
    pub atc_type: AtcType,
    pub vis_range: u32,
    pub rating: AtcRating,
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: i32,
}

impl Display for AtcPositionUpdateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let freqs_string = util::group_frequencies_without_symbol(&self.frequencies);
        write!(
            f,
            "%{}:{}:{}:{}:{}:{:.5}:{:.5}:{}",
            self.callsign,
            freqs_string,
            self.atc_type as u8,
            self.vis_range,
            self.rating as u8,
            self.latitude,
            self.longitude,
            self.elevation
        )
    }
}

impl TryFrom<&[&str]> for AtcPositionUpdateMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 7);
        let first = &fields[0][1..];
        Ok(AtcPositionUpdateMessage::new(
            first,
            util::split_frequencies(fields[1]),
            fields[2].parse()?,
            fields[3]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidVisRange(fields[3].to_string()))?,
            fields[4].parse()?,
            fields[5]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[5].to_string()))?,
            fields[6]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[6].to_string()))?,
            fields.get(7).unwrap_or(&"0").parse().unwrap_or_default(),
        ))
    }
}

impl AtcPositionUpdateMessage {
    pub fn new(
        callsign: impl AsRef<str>,
        frequencies: impl Into<Vec<RadioFrequency>>,
        atc_type: AtcType,
        vis_range: u32,
        rating: AtcRating,
        latitude: f64,
        longitude: f64,
        elevation: i32,
    ) -> Self {
        AtcPositionUpdateMessage {
            callsign: callsign.as_ref().to_uppercase(),
            frequencies: frequencies.into(),
            atc_type,
            vis_range,
            rating,
            latitude,
            longitude,
            elevation,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AtcSecondaryVisCentreMessage {
    pub callsign: String,
    pub index: usize,
    pub latitude: f64,
    pub longitude: f64,
}

impl Display for AtcSecondaryVisCentreMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}:{}:{:.5}:{:.5}",
            self.callsign, self.index, self.latitude, self.longitude
        )
    }
}

impl TryFrom<&[&str]> for AtcSecondaryVisCentreMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 4);
        let first = &fields[0][1..];
        Ok(AtcSecondaryVisCentreMessage::new(
            first,
            fields[1]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidIndex(fields[1].to_string()))?,
            fields[2]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[5].to_string()))?,
            fields[3]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[5].to_string()))?,
        ))
    }
}

impl AtcSecondaryVisCentreMessage {
    pub fn new(callsign: impl AsRef<str>, index: usize, latitude: f64, longitude: f64) -> Self {
        AtcSecondaryVisCentreMessage {
            callsign: callsign.as_ref().to_uppercase(),
            index,
            latitude,
            longitude,
        }
    }
}

/// Sent at regular intervals by a pilot client to update the server with its position
#[derive(Debug, Clone)]
pub struct PilotPositionUpdateMessage {
    pub callsign: String,
    pub transponder_mode: TransponderMode,
    pub transponder_code: TransponderCode,
    pub rating: PilotRating,
    pub latitude: f64,
    pub longitude: f64,
    pub true_altitude: f64,
    pub pressure_altitude: f64,
    pub ground_speed: u32,
    pub pitch: f64,
    pub bank: f64,
    pub heading: f64,
    pub on_ground: bool,
}

impl Display for PilotPositionUpdateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pbh =
            util::encode_pitch_bank_heading(self.pitch, self.bank, self.heading, self.on_ground);
        let alt_diff = self.pressure_altitude - self.true_altitude;
        write!(
            f,
            "@{}:{}:{}:{}:{:.5}:{:.5}:{}:{}:{}:{}",
            self.transponder_mode,
            self.callsign,
            self.transponder_code,
            self.rating as u8,
            self.latitude,
            self.longitude,
            self.true_altitude as i32,
            self.ground_speed,
            pbh,
            alt_diff as i32,
        )
    }
}

impl TryFrom<&[&str]> for PilotPositionUpdateMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 10);
        let first = &fields[0][1..];

        let true_altitude = fields[6]
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[6].to_string()))?;
        let alt_diff: f64 = fields[9]
            .parse()
            .map_err(|_| FsdMessageParseError::InvalidAltitudeDifference(fields[9].to_string()))?;
        let (pitch, bank, heading, on_ground) = {
            let pbh = fields[8].parse().map_err(|_| {
                FsdMessageParseError::InvalidPitchBankHeading(fields[8].to_string())
            })?;
            util::decode_pitch_bank_heading(pbh)
        };

        Ok(PilotPositionUpdateMessage::new(
            fields[1],
            first.parse()?,
            fields[2].parse()?,
            fields[3].parse()?,
            fields[4]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[4].to_string()))?,
            fields[5]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[5].to_string()))?,
            true_altitude,
            true_altitude + alt_diff,
            fields[7]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidSpeed(fields[7].to_string()))?,
            pitch,
            bank,
            heading,
            on_ground,
        ))
    }
}

impl PilotPositionUpdateMessage {
    pub fn new(
        callsign: impl AsRef<str>,
        transponder_mode: TransponderMode,
        transponder_code: TransponderCode,
        rating: PilotRating,
        latitude: f64,
        longitude: f64,
        true_altitude: f64,
        pressure_altitude: f64,
        ground_speed: u32,
        pitch: f64,
        bank: f64,
        heading: f64,
        on_ground: bool,
    ) -> Self {
        PilotPositionUpdateMessage {
            callsign: callsign.as_ref().to_uppercase(),
            transponder_mode,
            transponder_code,
            rating,
            latitude,
            longitude,
            true_altitude,
            pressure_altitude,
            ground_speed,
            pitch,
            bank,
            heading,
            on_ground,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuthenticationChallengeMessage {
    pub from: String,
    pub to: String,
    pub challenge: String,
}

impl Display for AuthenticationChallengeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$ZC{}:{}:{}", self.from, self.to, self.challenge)
    }
}

impl TryFrom<&[&str]> for AuthenticationChallengeMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];
        Ok(AuthenticationChallengeMessage::new(
            first, fields[1], fields[2],
        ))
    }
}

impl AuthenticationChallengeMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, challenge: impl Into<String>) -> Self {
        AuthenticationChallengeMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            challenge: challenge.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuthenticationResponseMessage {
    pub from: String,
    pub to: String,
    pub response: String,
}

impl Display for AuthenticationResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$ZR{}:{}:{}", self.from, self.to, self.response)
    }
}

impl TryFrom<&[&str]> for AuthenticationResponseMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];
        Ok(AuthenticationResponseMessage::new(
            first, fields[1], fields[2],
        ))
    }
}

impl AuthenticationResponseMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, response: impl Into<String>) -> Self {
        AuthenticationResponseMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            response: response.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextMessage {
    pub from: String,
    pub to: String,
    pub message: String,
}
impl Display for TextMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#TM{}:{}:{}", self.from, self.to, self.message)
    }
}
impl TryFrom<&[&str]> for TextMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];
        let mut message = fields[2].to_string();
        if fields.len() > 3 {
            for m in &fields[3..] {
                message.push(':');
                message.push_str(m);
            }
        }
        Ok(TextMessage::new(first, fields[1], message))
    }
}

impl TextMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, message: impl Into<String>) -> Self {
        TextMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FrequencyMessage {
    pub from: String,
    pub to: Vec<RadioFrequency>,
    pub message: String,
}

impl Display for FrequencyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frequencies = util::group_frequencies_with_symbol(&self.to);
        write!(f, "#TM{}:{}:{}", self.from, frequencies, self.message)
    }
}

impl TryFrom<&[&str]> for FrequencyMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];
        let mut message = fields[2].to_string();
        if fields.len() > 3 {
            for m in &fields[3..] {
                message.push(':');
                message.push_str(m);
            }
        }
        Ok(FrequencyMessage::new(
            first,
            util::split_frequencies(fields[1]),
            message,
        ))
    }
}

impl FrequencyMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl Into<Vec<RadioFrequency>>,
        message: impl Into<String>,
    ) -> Self {
        FrequencyMessage {
            from: from.as_ref().to_uppercase(),
            to: to.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChangeServerMessage {
    pub from: String,
    pub to: String,
    pub hostname: String,
}

impl Display for ChangeServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$XX{}:{}:{}", self.from, self.to, self.hostname)
    }
}

impl TryFrom<&[&str]> for ChangeServerMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];

        Ok(ChangeServerMessage::new(first, fields[1], fields[2]))
    }
}

impl ChangeServerMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, hostname: impl Into<String>) -> Self {
        ChangeServerMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            hostname: hostname.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InitialServerHandshakeMessage {
    pub from: String,
    pub to: String,
    pub version: String,
    pub initial_key: String,
}

impl Display for InitialServerHandshakeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "$DI{}:{}:{}:{}",
            self.from, self.to, self.version, self.initial_key
        )
    }
}

impl TryFrom<&[&str]> for InitialServerHandshakeMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];

        Ok(InitialServerHandshakeMessage::new(
            first, fields[1], fields[2], fields[3],
        ))
    }
}

impl InitialServerHandshakeMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        version: impl Into<String>,
        initial_key: impl Into<String>,
    ) -> Self {
        InitialServerHandshakeMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            version: version.into(),
            initial_key: initial_key.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InitialClientHandshakeMessage {
    pub from: String,
    pub to: String,
    pub client_id: u16,
    pub client_name: String,
    pub major_version: u32,
    pub minor_version: u32,
    pub cid: String,
    pub guid: String,
    pub initial_key: Option<String>,
}

impl Display for InitialClientHandshakeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = format!(
            "$ID{}:{}:{:04x}:{}:{}:{}:{}:{}",
            self.from,
            self.to,
            self.client_id,
            self.client_name,
            self.major_version,
            self.minor_version,
            self.cid,
            self.guid
        );
        if let Some(initial_key) = &self.initial_key {
            message.push(':');
            message.push_str(initial_key);
        }
        write!(f, "{message}")
    }
}

impl TryFrom<&[&str]> for InitialClientHandshakeMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 8);
        let first = &fields[0][3..];

        Ok(InitialClientHandshakeMessage::new(
            first,
            fields[1],
            u16::from_str_radix(fields[2], 16)
                .map_err(|_| FsdMessageParseError::InvalidClientID(fields[2].to_string()))?,
            fields[3],
            fields[4]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidVersionNumber(fields[4].to_string()))?,
            fields[5]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidVersionNumber(fields[5].to_string()))?,
            fields[6],
            fields[7],
            fields.get(8).copied(),
        ))
    }
}

impl InitialClientHandshakeMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        client_id: u16,
        client_name: impl Into<String>,
        major_version: u32,
        minor_version: u32,
        cid: impl Into<String>,
        guid: impl Into<String>,
        initial_key: Option<impl Into<String>>,
    ) -> Self {
        InitialClientHandshakeMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            client_id,
            client_name: client_name.into(),
            major_version,
            minor_version,
            cid: cid.into(),
            guid: guid.into(),
            initial_key: initial_key.map(|x| x.into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SendFastPositionUpdatesMessage {
    pub from: String,
    pub to: String,
    pub send_fast: bool,
}

impl Display for SendFastPositionUpdatesMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$SF{}:{}:{}", self.from, self.to, self.send_fast as u8)
    }
}

impl TryFrom<&[&str]> for SendFastPositionUpdatesMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];

        Ok(SendFastPositionUpdatesMessage::new(
            first,
            fields[1],
            fields[2] == "1",
        ))
    }
}

impl SendFastPositionUpdatesMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, send_fast: bool) -> Self {
        SendFastPositionUpdatesMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            send_fast,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VelocityPositionStoppedMessage {
    pub from: String,
    pub latitude: f64,
    pub longitude: f64,
    pub true_altitude: f64,
    pub altitude_agl: f64,
    pub pitch: f64,
    pub bank: f64,
    pub heading: f64,
    pub on_ground: bool,
    pub nose_gear_angle: Option<f64>,
}

impl Display for VelocityPositionStoppedMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pbh =
            util::encode_pitch_bank_heading(self.pitch, self.bank, self.heading, self.on_ground);
        write!(
            f,
            "#ST{}:{:.7}:{:.7}:{:.2}:{:.2}:{}",
            self.from, self.latitude, self.longitude, self.true_altitude, self.altitude_agl, pbh,
        )?;
        if let Some(nga) = self.nose_gear_angle {
            write!(f, ":{:.2}", nga)?;
        }
        Ok(())
    }
}

impl TryFrom<&[&str]> for VelocityPositionStoppedMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 6);
        let first = &fields[0][3..];
        let pbh = fields[5]
            .parse::<u32>()
            .map_err(|_| FsdMessageParseError::InvalidPitchBankHeading(fields[5].to_string()))?;
        let (pitch, bank, heading, on_ground) = util::decode_pitch_bank_heading(pbh);
        let nga =
            if let Some(nga) = fields.get(6) {
                Some(nga.parse::<f64>().map_err(|_| {
                    FsdMessageParseError::InvalidNosewheelAngle(fields[6].to_string())
                })?)
            } else {
                None
            };
        Ok(VelocityPositionStoppedMessage::new(
            first,
            fields[1]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[1].to_string()))?,
            fields[2]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[2].to_string()))?,
            fields[3]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[3].to_string()))?,
            fields[4]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[4].to_string()))?,
            pitch,
            bank,
            heading,
            on_ground,
            nga,
        ))
    }
}

impl VelocityPositionStoppedMessage {
    pub fn new(
        from: impl AsRef<str>,
        latitude: f64,
        longitude: f64,
        true_altitude: f64,
        altitude_agl: f64,
        pitch: f64,
        bank: f64,
        heading: f64,
        on_ground: bool,
        nose_gear_angle: Option<f64>,
    ) -> Self {
        VelocityPositionStoppedMessage {
            from: from.as_ref().to_uppercase(),
            latitude,
            longitude,
            true_altitude,
            altitude_agl,
            pitch,
            bank,
            heading,
            on_ground,
            nose_gear_angle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VelocityPositionSlowMessage {
    pub from: String,
    pub latitude: f64,
    pub longitude: f64,
    pub true_altitude: f64,
    pub altitude_agl: f64,
    pub pitch: f64,
    pub bank: f64,
    pub heading: f64,
    pub on_ground: bool,
    pub x_velocity: f64,
    pub y_velocity: f64,
    pub z_velocity: f64,
    pub pitch_rad_per_sec: f64,
    pub heading_rad_per_sec: f64,
    pub bank_rad_per_sec: f64,
    pub nose_gear_angle: Option<f64>,
}

impl Display for VelocityPositionSlowMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pbh =
            util::encode_pitch_bank_heading(self.pitch, self.bank, self.heading, self.on_ground);
        write!(
            f,
            "#SL{}:{:.7}:{:.7}:{:.2}:{:.2}:{}:{:.4}:{:.4}:{:.4}:{:.4}:{:.4}:{:.4}",
            self.from,
            self.latitude,
            self.longitude,
            self.true_altitude,
            self.altitude_agl,
            pbh,
            self.x_velocity,
            self.y_velocity,
            self.z_velocity,
            self.pitch_rad_per_sec,
            self.heading_rad_per_sec,
            self.bank_rad_per_sec,
        )?;
        if let Some(nga) = self.nose_gear_angle {
            write!(f, ":{:.2}", nga)?;
        }
        Ok(())
    }
}

impl TryFrom<&[&str]> for VelocityPositionSlowMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 12);

        let first = &fields[0][3..];
        let pbh = fields[5]
            .parse::<u32>()
            .map_err(|_| FsdMessageParseError::InvalidPitchBankHeading(fields[5].to_string()))?;
        let (pitch, bank, heading, on_ground) = util::decode_pitch_bank_heading(pbh);
        let nga =
            if let Some(nga) = fields.get(12) {
                Some(nga.parse::<f64>().map_err(|_| {
                    FsdMessageParseError::InvalidNosewheelAngle(fields[12].to_string())
                })?)
            } else {
                None
            };
        Ok(VelocityPositionSlowMessage::new(
            first,
            fields[1]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[1].to_string()))?,
            fields[2]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[2].to_string()))?,
            fields[3]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[3].to_string()))?,
            fields[4]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[4].to_string()))?,
            pitch,
            bank,
            heading,
            on_ground,
            fields[6].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[6].to_string())
            })?,
            fields[7].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[7].to_string())
            })?,
            fields[8].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[8].to_string())
            })?,
            fields[9].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[9].to_string())
            })?,
            fields[10].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[10].to_string())
            })?,
            fields[11].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[11].to_string())
            })?,
            nga,
        ))
    }
}

impl VelocityPositionSlowMessage {
    pub fn new(
        from: impl AsRef<str>,
        latitude: f64,
        longitude: f64,
        true_altitude: f64,
        altitude_agl: f64,
        pitch: f64,
        bank: f64,
        heading: f64,
        on_ground: bool,
        x_velocity: f64,
        y_velocity: f64,
        z_velocity: f64,
        pitch_rad_per_sec: f64,
        heading_rad_per_sec: f64,
        bank_rad_per_sec: f64,
        nose_gear_angle: Option<f64>,
    ) -> Self {
        VelocityPositionSlowMessage {
            from: from.as_ref().to_uppercase(),
            latitude,
            longitude,
            true_altitude,
            altitude_agl,
            pitch,
            bank,
            heading,
            on_ground,
            x_velocity,
            y_velocity,
            z_velocity,
            pitch_rad_per_sec,
            heading_rad_per_sec,
            bank_rad_per_sec,
            nose_gear_angle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VelocityPositionFastMessage {
    pub from: String,
    pub latitude: f64,
    pub longitude: f64,
    pub true_altitude: f64,
    pub altitude_agl: f64,
    pub pitch: f64,
    pub bank: f64,
    pub heading: f64,
    pub on_ground: bool,
    pub x_velocity: f64,
    pub y_velocity: f64,
    pub z_velocity: f64,
    pub pitch_rad_per_sec: f64,
    pub heading_rad_per_sec: f64,
    pub bank_rad_per_sec: f64,
    pub nose_gear_angle: Option<f64>,
}

impl From<VelocityPositionSlowMessage> for VelocityPositionFastMessage {
    fn from(value: VelocityPositionSlowMessage) -> Self {
        VelocityPositionFastMessage {
            from: value.from,
            latitude: value.latitude,
            longitude: value.longitude,
            true_altitude: value.true_altitude,
            altitude_agl: value.altitude_agl,
            pitch: value.pitch,
            bank: value.bank,
            heading: value.heading,
            on_ground: value.on_ground,
            x_velocity: value.x_velocity,
            y_velocity: value.y_velocity,
            z_velocity: value.z_velocity,
            pitch_rad_per_sec: value.pitch_rad_per_sec,
            heading_rad_per_sec: value.heading_rad_per_sec,
            bank_rad_per_sec: value.bank_rad_per_sec,
            nose_gear_angle: value.nose_gear_angle,
        }
    }
}

impl Display for VelocityPositionFastMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pbh =
            util::encode_pitch_bank_heading(self.pitch, self.bank, self.heading, self.on_ground);
        write!(
            f,
            "^{}:{:.7}:{:.7}:{:.2}:{:.2}:{}:{:.4}:{:.4}:{:.4}:{:.4}:{:.4}:{:.4}",
            self.from,
            self.latitude,
            self.longitude,
            self.true_altitude,
            self.altitude_agl,
            pbh,
            self.x_velocity,
            self.y_velocity,
            self.z_velocity,
            self.pitch_rad_per_sec,
            self.heading_rad_per_sec,
            self.bank_rad_per_sec,
        )?;
        if let Some(nga) = self.nose_gear_angle {
            write!(f, ":{:.2}", nga)?;
        }
        Ok(())
    }
}

impl TryFrom<&[&str]> for VelocityPositionFastMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 12);

        let first = &fields[0][1..];
        let pbh = fields[5]
            .parse::<u32>()
            .map_err(|_| FsdMessageParseError::InvalidPitchBankHeading(fields[5].to_string()))?;
        let (pitch, bank, heading, on_ground) = util::decode_pitch_bank_heading(pbh);
        let nga =
            if let Some(nga) = fields.get(12) {
                Some(nga.parse::<f64>().map_err(|_| {
                    FsdMessageParseError::InvalidNosewheelAngle(fields[12].to_string())
                })?)
            } else {
                None
            };
        Ok(VelocityPositionFastMessage::new(
            first,
            fields[1]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[1].to_string()))?,
            fields[2]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidCoordinate(fields[2].to_string()))?,
            fields[3]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[3].to_string()))?,
            fields[4]
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidAltitude(fields[4].to_string()))?,
            pitch,
            bank,
            heading,
            on_ground,
            fields[6].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[6].to_string())
            })?,
            fields[7].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[7].to_string())
            })?,
            fields[8].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[8].to_string())
            })?,
            fields[9].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[9].to_string())
            })?,
            fields[10].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[10].to_string())
            })?,
            fields[11].parse().map_err(|_| {
                FsdMessageParseError::InvalidPositionVelocity(fields[11].to_string())
            })?,
            nga,
        ))
    }
}

impl VelocityPositionFastMessage {
    pub fn new(
        from: impl AsRef<str>,
        latitude: f64,
        longitude: f64,
        true_altitude: f64,
        altitude_agl: f64,
        pitch: f64,
        bank: f64,
        heading: f64,
        on_ground: bool,
        x_velocity: f64,
        y_velocity: f64,
        z_velocity: f64,
        pitch_rad_per_sec: f64,
        heading_rad_per_sec: f64,
        bank_rad_per_sec: f64,
        nose_gear_angle: Option<f64>,
    ) -> Self {
        VelocityPositionFastMessage {
            from: from.as_ref().to_uppercase(),
            latitude,
            longitude,
            true_altitude,
            altitude_agl,
            pitch,
            bank,
            heading,
            on_ground,
            x_velocity,
            y_velocity,
            z_velocity,
            pitch_rad_per_sec,
            heading_rad_per_sec,
            bank_rad_per_sec,
            nose_gear_angle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KillMessage {
    pub from: String,
    pub to: String,
    pub reason: Option<String>,
}

impl Display for KillMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = format!("$!!{}:{}", self.from, self.to);
        if let Some(reason) = &self.reason {
            message.push(':');
            message.push_str(reason);
        };
        write!(f, "{}", message)
    }
}

impl TryFrom<&[&str]> for KillMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 2);
        let first = &fields[0][3..];

        Ok(KillMessage::new(first, fields[1], fields.get(2).copied()))
    }
}

impl KillMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        reason: Option<impl Into<String>>,
    ) -> Self {
        KillMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            reason: reason.map(|x| x.into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetarRequestMessage {
    pub from: String,
    pub to: String,
    pub station: String,
}

impl Display for MetarRequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$AX{}:{}:METAR:{}", self.from, self.to, self.station)
    }
}

impl TryFrom<&[&str]> for MetarRequestMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 4);
        let first = &fields[0][3..];

        Ok(MetarRequestMessage::new(first, fields[1], fields[3]))
    }
}

impl MetarRequestMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, station: impl AsRef<str>) -> Self {
        MetarRequestMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            station: station.as_ref().to_uppercase(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetarResponseMessage {
    pub from: String,
    pub to: String,
    pub metar: String,
}

impl Display for MetarResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$AR{}:{}:METAR:{}", self.from, self.to, self.metar)
    }
}

impl TryFrom<&[&str]> for MetarResponseMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 4);
        let first = &fields[0][3..];

        Ok(MetarResponseMessage::new(first, fields[1], fields[3]))
    }
}

impl MetarResponseMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, metar: impl AsRef<str>) -> Self {
        MetarResponseMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            metar: metar.as_ref().to_uppercase(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PingMessage {
    pub from: String,
    pub to: String,
    pub data: String,
}

impl Display for PingMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$PI{}:{}:{}", self.from, self.to, self.data)
    }
}

impl TryFrom<&[&str]> for PingMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];

        Ok(PingMessage::new(first, fields[1], fields[2]))
    }
}

impl PingMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        PingMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            data: data.as_ref().to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PongMessage {
    pub from: String,
    pub to: String,
    pub data: String,
}

impl Display for PongMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$PO{}:{}:{}", self.from, self.to, self.data)
    }
}

impl TryFrom<&[&str]> for PongMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];

        Ok(PongMessage::new(first, fields[1], fields[2]))
    }
}

impl PongMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        PongMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            data: data.as_ref().to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlaneInfoRequestMessage {
    pub from: String,
    pub to: String,
}

impl Display for PlaneInfoRequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#SB{}:{}:PIR", self.from, self.to)
    }
}

impl TryFrom<&[&str]> for PlaneInfoRequestMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];

        Ok(PlaneInfoRequestMessage::new(first, fields[1]))
    }
}

impl PlaneInfoRequestMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>) -> Self {
        PlaneInfoRequestMessage {
            from: from.as_ref().into(),
            to: to.as_ref().into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlaneInfoResponseMessage {
    pub from: String,
    pub to: String,
    pub plane_info: PlaneInfo,
}

impl Display for PlaneInfoResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#SB{}:{}:PI:GEN:{}", self.from, self.to, self.plane_info)
    }
}

impl TryFrom<&[&str]> for PlaneInfoResponseMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 5);
        let first = &fields[0][3..];
        if fields[3] != "GEN" {
            return Err(FsdMessageParseError::UnknownMessageType(fields.join(":")));
        }

        Ok(PlaneInfoResponseMessage::new(
            first,
            fields[1],
            fields[4..].into(),
        ))
    }
}

impl PlaneInfoResponseMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, plane_info: PlaneInfo) -> Self {
        PlaneInfoResponseMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            plane_info,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FsdErrorMessage {
    pub from: String,
    pub to: String,
    pub error_type: FsdError,
}

impl Display for FsdErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let FsdError::Other(ref message) = self.error_type {
            write!(
                f,
                "$ER{}:{}:{:03}::{}",
                self.from,
                self.to,
                self.error_type.error_number(),
                message
            )
        } else {
            write!(
                f,
                "$ER{}:{}:{:03}::",
                self.from,
                self.to,
                self.error_type.error_number()
            )
        }
    }
}

impl TryFrom<&[&str]> for FsdErrorMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 5);
        let first = &fields[0][3..];
        let error_type = match fields[2]
            .parse::<u8>()
            .map_err(|_| FsdMessageParseError::InvalidServerError(fields[2].to_string()))?
        {
            1 => FsdError::CallsignInUse,
            2 => FsdError::InvalidCallsign,
            3 => FsdError::AlreadyRegistered,
            4 => FsdError::SyntaxError,
            5 => FsdError::InvalidCallsign,
            6 => FsdError::InvalidCidPassword,
            7 => FsdError::NoSuchCallsign(fields[3].to_uppercase()),
            8 => FsdError::NoFlightPlan(fields[3].to_uppercase()),
            9 => FsdError::NoWeatherProfile(fields[3].to_uppercase()),
            10 => FsdError::InvalidProtocolRevision,
            11 => FsdError::RequestedLevelTooHigh,
            12 => FsdError::ServerFull,
            13 => FsdError::CertificateSuspended,
            14 => FsdError::InvalidControl,
            15 => FsdError::InvalidPositionForRating,
            16 => FsdError::UnauthorisedClient,
            17 => FsdError::AuthTimeOut,
            _ => FsdError::Other(fields[4].to_string()),
        };
        Ok(FsdErrorMessage::new(first, fields[1], error_type))
    }
}

impl FsdErrorMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, error_type: FsdError) -> Self {
        FsdErrorMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            error_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlightPlanMessage {
    pub to: String,
    pub callsign: String,
    pub flight_plan: FlightPlan,
}

//$FP(CALLSIGN):(RECIPIENT):(FLIGHT RULES):(AC TYPE):(FILED SPEED):(ORIGIN):(SCHEDULED DEPARTURE TIME):(ACTUAL DEPARTURE TIME):
//(CRUISE LEVEL):(DESTINATION):(HOURS ENROUTE):(MINS ENROUTE):(HOURS FUEL):(MINS FUEL):(ALTERNATE):(REMARKS)

impl Display for FlightPlanMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$FP{}:{}:{}", self.callsign, self.to, self.flight_plan,)
    }
}

impl TryFrom<&[&str]> for FlightPlanMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_exact_num_fields!(fields, 17);
        let first = &fields[0][3..];

        Ok(FlightPlanMessage::new(
            fields[1],
            first,
            fields[2..].try_into()?,
        ))
    }
}

impl FlightPlanMessage {
    pub fn new(to: impl AsRef<str>, callsign: impl AsRef<str>, flight_plan: FlightPlan) -> Self {
        FlightPlanMessage {
            to: to.as_ref().to_uppercase(),
            callsign: callsign.as_ref().to_uppercase(),
            flight_plan,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlightPlanAmendmentMessage {
    pub from: String,
    pub to: String,
    pub callsign: String,
    pub flight_plan: FlightPlan,
}

//$FP(CALLSIGN):(RECIPIENT):(FLIGHT RULES):(AC TYPE):(FILED SPEED):(ORIGIN):(SCHEDULED DEPARTURE TIME):(ACTUAL DEPARTURE TIME):
//(CRUISE LEVEL):(DESTINATION):(HOURS ENROUTE):(MINS ENROUTE):(HOURS FUEL):(MINS FUEL):(ALTERNATE):(REMARKS)

impl Display for FlightPlanAmendmentMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "$AM{}:{}:{}:{}",
            self.from, self.to, self.callsign, self.flight_plan,
        )
    }
}

impl TryFrom<&[&str]> for FlightPlanAmendmentMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_exact_num_fields!(fields, 18);
        let first = &fields[0][3..];
        Ok(FlightPlanAmendmentMessage::new(
            first,
            fields[1],
            fields[2],
            fields[3..].try_into()?,
        ))
    }
}

impl FlightPlanAmendmentMessage {
    pub fn new(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        callsign: impl AsRef<str>,
        flight_plan: FlightPlan,
    ) -> Self {
        FlightPlanAmendmentMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            callsign: callsign.as_ref().to_uppercase(),
            flight_plan,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ClientQueryMessage {
    pub from: String,
    pub to: String,
    pub query_type: ClientQueryType,
}

impl Display for ClientQueryMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$CQ{}:{}:{}", self.from, self.to, self.query_type)
    }
}
impl TryFrom<&[&str]> for ClientQueryMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 3);
        let first = &fields[0][3..];
        match fields[2] {
            "C?" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::Com1Freq,
            )),
            "IP" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::PublicIP,
            )),
            "ATIS" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::ATIS,
            )),
            "RN" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::RealName,
            )),
            "IPC" => {
                //$CQserver:N194Q:IPC:W:852:8704
                let remainder = fields
                    .get(3..6)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(6, 3))?;
                if remainder[0] != "W" || remainder[1] != "852" {
                    return Err(FsdMessageParseError::UnknownMessageType(format!(
                        "IPC:{}:{}:{}",
                        fields[0], fields[1], fields[2]
                    )));
                }
                let code = TransponderCode::try_from_bcd_format(remainder[2])?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::ForceBeaconCode { code },
                ))
            }
            "SV" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::Server,
            )),
            "ACC" => {
                let data = fields
                    .get(3)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(4, 3))?;
                if data.contains("request") {
                    Ok(ClientQueryMessage::new(
                        first,
                        fields[1],
                        ClientQueryType::AircraftConfigurationRequest,
                    ))
                } else {
                    let data = {
                        let mut data_string = String::new();
                        let mut fields_peekable = fields[3..].iter().peekable();
                        while let Some(field) = fields_peekable.next() {
                            data_string.push_str(field);
                            if fields_peekable.peek().is_some() {
                                data_string.push(':');
                            }
                        }
                        data_string
                    };
                    Ok(ClientQueryMessage::new(
                        first,
                        fields[1],
                        ClientQueryType::AircraftConfigurationResponse {
                            aircraft_config: data.as_str().parse()?,
                        },
                    ))
                }
            }
            "BY" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::RequestRelief,
            )),
            "HLP" => {
                let mut message = fields.get(3).map(|s| s.to_string());
                if let Some(ref msg) = message
                    && msg.is_empty()
                {
                    message = None;
                }
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::HelpRequest { message },
                ))
            }
            "NOHLP" => {
                let mut message = fields.get(3).map(|s| s.to_string());
                if let Some(ref msg) = message
                    && msg.is_empty()
                {
                    message = None;
                }
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::CancelHelpRequest { message },
                ))
            }
            "SC" => {
                check_min_num_fields!(fields, 5);
                let contents = fields[4].parse()?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SetScratchpad {
                        aircraft_callsign: fields[3].to_uppercase(),
                        contents,
                    },
                ))
            }
            "FA" => {
                check_min_num_fields!(fields, 5);
                let level = fields[4].parse()?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SetFinalAltitude {
                        aircraft_callsign: fields[3].to_uppercase(),
                        level,
                    },
                ))
            }
            "BC" => {
                check_min_num_fields!(fields, 5);
                let code = fields[4].parse()?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SetBeaconCode {
                        aircraft_callsign: fields[3].to_uppercase(),
                        code,
                    },
                ))
            }
            "ATC" => {
                let atc_callsign = fields.get(3).unwrap_or(&first).to_uppercase();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::IsValidATC { atc_callsign },
                ))
            }
            "FP" => {
                let aircraft_callsign = fields
                    .get(3)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(4, 3))?
                    .to_uppercase();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::FlightPlan { aircraft_callsign },
                ))
            }
            "NEWATIS" => {
                check_min_num_fields!(fields, 5);
                let (atis_letter, surface_wind, pressure) = util::parse_new_atis(&fields[3..])?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::NewATIS {
                        atis_letter,
                        surface_wind,
                        pressure,
                    },
                ))
            }
            // $CQLFMD_ATIS:@94835:NEWINFO:O
            "NEWINFO" => {
                check_min_num_fields!(fields, 4);
                let atis_letter = fields[3]
                    .chars()
                    .last()
                    .and_then(|c| {
                        let c = c.to_ascii_uppercase();
                        if (c as u8) < 65 || (c as u8) > 90 {
                            None
                        } else {
                            Some(c)
                        }
                    })
                    .ok_or_else(|| FsdMessageParseError::InvalidATISLine(fields.join(":")))?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::NewInfo { atis_letter },
                ))
            }
            "VT" => {
                check_min_num_fields!(fields, 5);
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SetVoiceType {
                        aircraft_callsign: fields[3].to_uppercase(),
                        voice_capability: fields[4].parse()?,
                    },
                ))
            }
            "WH" => {
                check_min_num_fields!(fields, 4);
                let aircraft_callsign = fields[3].to_uppercase();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::WhoHas { aircraft_callsign },
                ))
            }
            "TA" => {
                check_min_num_fields!(fields, 5);
                let aircraft_callsign = fields[3].to_uppercase();
                let level = fields[4].parse()?;
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SetTempAltitude {
                        aircraft_callsign,
                        level,
                    },
                ))
            }
            "HT" => {
                check_min_num_fields!(fields, 5);
                let aircraft_callsign = fields[3].to_uppercase();
                let atc_callsign = fields[4].to_uppercase();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::AcceptHandoff {
                        aircraft_callsign,
                        atc_callsign,
                    },
                ))
            }
            "DR" => {
                check_min_num_fields!(fields, 4);
                let aircraft_callsign = fields[3].to_uppercase();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::DropTrack { aircraft_callsign },
                ))
            }
            "CAPS" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::Capabilities,
            )),
            "IT" => {
                check_min_num_fields!(fields, 4);
                let aircraft_callsign = fields[3].to_uppercase();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::InitiateTrack { aircraft_callsign },
                ))
            }
            "HI" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::CancelRequestRelief,
            )),
            "INF" => Ok(ClientQueryMessage::new(
                first,
                fields[1],
                ClientQueryType::INF,
            )),
            "SIMTIME" => {
                check_min_num_fields!(fields, 4);
                let time = match NaiveDateTime::parse_from_str(fields[3], "%Y%m%d%H%M%S") {
                    Ok(naive_time) => naive_time.and_utc(),
                    Err(e) => {
                        return Err(FsdMessageParseError::InvalidTime(format!(
                            "SIMTIME uses incorrect format: {}, {e}",
                            fields[3]
                        )));
                    }
                };
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SimTime { time },
                ))
            }
            "GD" => {
                check_min_num_fields!(fields, 5);
                let aircraft_callsign = fields[4].to_uppercase();
                let contents = fields[4].to_string();
                Ok(ClientQueryMessage::new(
                    first,
                    fields[1],
                    ClientQueryType::SetGlobalData {
                        aircraft_callsign,
                        contents,
                    },
                ))
            }
            _ => Err(FsdMessageParseError::UnknownMessageType(
                fields[2].to_string(),
            )),
        }
    }
}
impl ClientQueryMessage {
    fn new(from: impl AsRef<str>, to: impl AsRef<str>, query_type: ClientQueryType) -> Self {
        ClientQueryMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            query_type,
        }
    }
    pub fn force_beacon_code(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        code: TransponderCode,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::ForceBeaconCode { code })
    }
    pub fn help_request(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        message: Option<impl AsRef<str>>,
    ) -> ClientQueryMessage {
        let message = message.map(|msg| msg.as_ref().to_string());
        ClientQueryMessage::new(from, to, ClientQueryType::HelpRequest { message })
    }

    pub fn cancel_help_request(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        message: Option<impl AsRef<str>>,
    ) -> ClientQueryMessage {
        let message = message.map(|msg| msg.as_ref().to_string());
        ClientQueryMessage::new(from, to, ClientQueryType::CancelHelpRequest { message })
    }

    pub fn com_1_freq(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::Com1Freq)
    }
    pub fn public_ip(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::PublicIP)
    }
    pub fn atis(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::ATIS)
    }
    pub fn real_name(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::RealName)
    }
    pub fn server(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::Server)
    }
    pub fn capabilities(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::Capabilities)
    }
    pub fn is_valid_atc(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        atc_callsign: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::IsValidATC {
                atc_callsign: atc_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn client_information(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::INF)
    }
    pub fn flight_plan(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::FlightPlan {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn request_relief(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::RequestRelief)
    }
    pub fn cancel_request_relief(from: impl AsRef<str>, to: impl AsRef<str>) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::CancelRequestRelief)
    }
    pub fn who_has(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::WhoHas {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn initiate_track(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::InitiateTrack {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn accept_handoff(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        atc_callsign: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::AcceptHandoff {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                atc_callsign: atc_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn drop_track(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::DropTrack {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn set_final_altitude(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        level: Level,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::SetFinalAltitude {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                level,
            },
        )
    }
    pub fn set_temp_altitude(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        level: Level,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::SetTempAltitude {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                level,
            },
        )
    }
    pub fn set_beacon_code(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        code: TransponderCode,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::SetBeaconCode {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                code,
            },
        )
    }
    pub fn set_scratchpad(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        contents: ScratchPad,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::SetScratchpad {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                contents,
            },
        )
    }
    pub fn set_global_data(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        contents: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::SetGlobalData {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                contents: contents.as_ref().to_string(),
            },
        )
    }
    pub fn set_voice_type(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        voice_capability: VoiceCapability,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::SetVoiceType {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                voice_capability,
            },
        )
    }
    pub fn aircraft_config_request(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::AircraftConfigurationRequest)
    }
    pub fn aircraft_config_response(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_config: AircraftConfig,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::AircraftConfigurationResponse { aircraft_config },
        )
    }
    pub fn new_info(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        atis_letter: char,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(from, to, ClientQueryType::NewInfo { atis_letter })
    }
    pub fn new_atis(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        atis_letter: char,
        wind_dir_and_speed: impl AsRef<str>,
        pressure: impl AsRef<str>,
    ) -> ClientQueryMessage {
        ClientQueryMessage::new(
            from,
            to,
            ClientQueryType::NewATIS {
                atis_letter,
                surface_wind: wind_dir_and_speed.as_ref().to_uppercase(),
                pressure: pressure.as_ref().to_uppercase(),
            },
        )
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ClientQueryResponseMessage {
    pub from: String,
    pub to: String,
    pub response_type: ClientResponseType,
}

impl Display for ClientQueryResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$CR{}:{}:{}", self.from, self.to, self.response_type)
    }
}
impl TryFrom<&[&str]> for ClientQueryResponseMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 4);

        let from = &fields[0][3..];
        let to = fields[1];
        let response_type = match fields[2] {
            "C?" => ClientResponseType::Com1Freq {
                frequency: RadioFrequency::try_from_human_readable_string(fields[3])?,
            },
            "ATIS" => {
                check_min_num_fields!(fields, 5);
                match fields[3] {
                    "V" => ClientResponseType::ATIS {
                        atis_line: AtisLine::VoiceServer(fields[4].to_string()),
                    },
                    "T" => {
                        let message = util::assemble_with_colons(&fields[4..]);
                        ClientResponseType::ATIS {
                            atis_line: AtisLine::TextLine(message),
                        }
                    }
                    "Z" => {
                        let logoff_time = if fields[4].ends_with('z') {
                            &fields[4][..fields[4].len() - 1]
                        } else {
                            fields[4]
                        };
                        ClientResponseType::ATIS {
                            atis_line: AtisLine::LogoffTime(logoff_time.parse().ok()),
                        }
                    }
                    "E" => {
                        let line_count: usize = fields[4].parse().map_err(|_| {
                            FsdMessageParseError::InvalidATISLine(fields[4].to_string())
                        })?;
                        ClientResponseType::ATIS {
                            atis_line: AtisLine::EndMarker(line_count),
                        }
                    }
                    _ => return Err(FsdMessageParseError::InvalidATISLine(fields[3].to_string())),
                }
            }
            "RN" => {
                check_min_num_fields!(fields, 4);
                let name = fields[3].to_string();
                let sector_file: String = fields[4].to_string();
                let rating: u8 = fields[5]
                    .parse()
                    .map_err(|_| FsdMessageParseError::InvalidRating(fields[5].to_string()))?;
                ClientResponseType::RealName {
                    name,
                    sector_file,
                    rating,
                }
            }
            "IP" => ClientResponseType::PublicIP {
                ip_address: fields
                    .get(3)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(4, fields.len()))?
                    .to_string(),
            },
            "SV" => ClientResponseType::Server {
                hostname_or_ip_address: fields
                    .get(3)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(4, fields.len()))?
                    .to_string(),
            },
            "ATC" => {
                check_min_num_fields!(fields, 4);
                let valid_atc = match fields[3].to_uppercase().as_str() {
                    "Y" => true,
                    "N" => false,
                    _ => {
                        return Err(FsdMessageParseError::InvalidValidAtcStatus(
                            fields[3].to_string(),
                        ));
                    }
                };
                let atc_callsign = fields.get(4).unwrap_or(&fields[1]).to_uppercase();
                ClientResponseType::IsValidATC {
                    atc_callsign,
                    valid_atc,
                }
            }
            "CAPS" => {
                check_min_num_fields!(fields, 4);
                let capabilities = util::read_capabilities(&fields[3..]);
                ClientResponseType::Capabilities { capabilities }
            }
            _ => {
                return Err(FsdMessageParseError::UnknownMessageType(
                    fields[2].to_string(),
                ));
            }
        };
        Ok(ClientQueryResponseMessage::new(from, to, response_type))
    }
}
impl ClientQueryResponseMessage {
    fn new(from: impl AsRef<str>, to: impl AsRef<str>, response_type: ClientResponseType) -> Self {
        ClientQueryResponseMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            response_type,
        }
    }

    pub fn com_1_freq(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        frequency: RadioFrequency,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(from, to, ClientResponseType::Com1Freq { frequency })
    }
    pub fn atis(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        atis_line: AtisLine,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(from, to, ClientResponseType::ATIS { atis_line })
    }
    pub fn real_name(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        name: impl Into<String>,
        sector_file: impl Into<String>,
        rating: u8,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(
            from,
            to,
            ClientResponseType::RealName {
                name: name.into(),
                sector_file: sector_file.into(),
                rating,
            },
        )
    }
    pub fn capabilities(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        capabilities: impl Into<Vec<ClientCapability>>,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(
            from,
            to,
            ClientResponseType::Capabilities {
                capabilities: capabilities.into(),
            },
        )
    }
    pub fn public_ip(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        ip_address: impl Into<String>,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(
            from,
            to,
            ClientResponseType::PublicIP {
                ip_address: ip_address.into(),
            },
        )
    }
    pub fn server(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        hostname_or_ip_address: impl Into<String>,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(
            from,
            to,
            ClientResponseType::Server {
                hostname_or_ip_address: hostname_or_ip_address.into(),
            },
        )
    }
    pub fn is_valid_atc(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        atc_callsign: impl AsRef<str>,
        valid: bool,
    ) -> ClientQueryResponseMessage {
        ClientQueryResponseMessage::new(
            from,
            to,
            ClientResponseType::IsValidATC {
                atc_callsign: atc_callsign.as_ref().to_uppercase(),
                valid_atc: valid,
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct HandoffOfferMessage {
    pub from: String,
    pub to: String,
    pub aircraft: String,
}

impl Display for HandoffOfferMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$HO{}:{}:{}", self.from, self.to, self.aircraft)
    }
}

impl TryFrom<&[&str]> for HandoffOfferMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        if fields.len() < 3 {
            return Err(FsdMessageParseError::InvalidFieldCount(3, fields.len()));
        }
        let first = &fields[0][3..];
        Ok(HandoffOfferMessage::new(first, fields[1], fields[2]))
    }
}

impl HandoffOfferMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, aircraft: impl AsRef<str>) -> Self {
        HandoffOfferMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            aircraft: aircraft.as_ref().to_uppercase(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct SharedStateMessage {
    pub from: String,
    pub to: String,
    pub shared_state_type: SharedStateType,
}

impl Display for SharedStateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#PC{}:{}:CCP:{}",
            self.from, self.to, self.shared_state_type
        )
    }
}
impl TryFrom<&[&str]> for SharedStateMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        check_min_num_fields!(fields, 4);
        let from = &fields[0][3..];
        let to = fields[1];
        let shared_state_type = match fields[3] {
            "VER" => SharedStateType::Version,
            "ID" => SharedStateType::ID,
            "DI" => SharedStateType::DI,
            "IH" => SharedStateType::IHave {
                aircraft_callsign: fields
                    .get(4)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(5, fields.len()))?
                    .to_uppercase(),
            },
            "SC" => {
                check_min_num_fields!(fields, 6);
                let scratchpad_contents = fields[5].parse()?;
                SharedStateType::ScratchPad {
                    aircraft_callsign: fields[4].to_uppercase(),
                    contents: scratchpad_contents,
                }
            }
            "GD" => {
                check_min_num_fields!(fields, 6);
                let aircraft_callsign = fields[4].to_uppercase();
                let contents = fields[5].to_string();
                SharedStateType::GlobalData {
                    aircraft_callsign,
                    contents,
                }
            }
            "TA" => {
                check_min_num_fields!(fields, 6);
                let level = fields[5].parse()?;
                SharedStateType::TempAltitude {
                    aircraft_callsign: fields[4].to_uppercase(),
                    level,
                }
            }
            "FA" => {
                check_min_num_fields!(fields, 6);
                let level = fields[5].parse()?;
                SharedStateType::FinalAltitude {
                    aircraft_callsign: fields[4].to_uppercase(),
                    level,
                }
            }
            "VT" => {
                check_min_num_fields!(fields, 6);
                let voice_capability: VoiceCapability = fields[5].parse()?;
                SharedStateType::VoiceType {
                    aircraft_callsign: fields[4].to_uppercase(),
                    voice_capability,
                }
            }
            "BC" => {
                check_min_num_fields!(fields, 4);
                let code: TransponderCode = fields[5].parse()?;
                SharedStateType::BeaconCode {
                    aircraft_callsign: fields[4].to_uppercase(),
                    code,
                }
            }
            "HC" => SharedStateType::HandoffCancel {
                aircraft_callsign: fields
                    .get(4)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(5, fields.len()))?
                    .to_uppercase(),
            },
            "PT" => SharedStateType::PointOut {
                aircraft_callsign: fields
                    .get(4)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(5, fields.len()))?
                    .to_uppercase(),
            },
            "DP" => SharedStateType::PushToDepartureList {
                aircraft_callsign: fields
                    .get(4)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(5, fields.len()))?
                    .to_uppercase(),
            },
            "ST" => {
                let aircraft_callsign = fields
                    .get(4)
                    .ok_or(FsdMessageParseError::InvalidFieldCount(5, fields.len()))?
                    .to_uppercase();
                let format = fields.get(5).and_then(|i| i.parse::<i32>().ok());
                let contents = fields
                    .get(6..)
                    .map(|c| c.iter().map(|e| e.to_string()).collect::<Vec<_>>());
                SharedStateType::FlightStrip {
                    aircraft_callsign,
                    format,
                    contents,
                }
            }
            "IC" | "IK" | "IB" | "EC" | "OV" | "OK" | "OB" | "EO" | "MN" | "MK" | "MB" | "EM" => {
                let ip_address = fields
                    .get(4)
                    .and_then(|ip| ip.parse::<Ipv4Addr>().ok())
                    .ok_or_else(|| {
                        FsdMessageParseError::InvalidIPAddress(
                            fields.get(4).map(|x| x.to_string()).unwrap_or_default(),
                        )
                    });
                let port = fields
                    .get(5)
                    .and_then(|port| port.parse::<u16>().ok())
                    .ok_or_else(|| {
                        FsdMessageParseError::InvalidPort(
                            fields.get(5).map(|x| x.to_string()).unwrap_or_default(),
                        )
                    });
                let (landline_type, landline_command) = match fields[3] {
                    "IC" => (
                        LandLineType::Intercom,
                        LandLineCommand::Request {
                            ip_address: ip_address?,
                            port: port?,
                        },
                    ),
                    "IK" => (
                        LandLineType::Intercom,
                        LandLineCommand::Approve {
                            ip_address: ip_address?,
                            port: port?,
                        },
                    ),
                    "IB" => (LandLineType::Intercom, LandLineCommand::Reject),
                    "EC" => (LandLineType::Intercom, LandLineCommand::End),

                    "OV" => (
                        LandLineType::Override,
                        LandLineCommand::Request {
                            ip_address: ip_address?,
                            port: port?,
                        },
                    ),
                    "OK" => (
                        LandLineType::Override,
                        LandLineCommand::Approve {
                            ip_address: ip_address?,
                            port: port?,
                        },
                    ),
                    "OB" => (LandLineType::Override, LandLineCommand::Reject),
                    "EO" => (LandLineType::Override, LandLineCommand::End),

                    "MN" => (
                        LandLineType::Monitor,
                        LandLineCommand::Request {
                            ip_address: ip_address?,
                            port: port?,
                        },
                    ),
                    "MK" => (
                        LandLineType::Monitor,
                        LandLineCommand::Approve {
                            ip_address: ip_address?,
                            port: port?,
                        },
                    ),
                    "MB" => (LandLineType::Monitor, LandLineCommand::Reject),
                    "EM" => (LandLineType::Monitor, LandLineCommand::End),

                    _ => unreachable!(),
                };
                SharedStateType::LandLine {
                    landline_type,
                    landline_command,
                }
            }
            _ => {
                return Err(FsdMessageParseError::InvalidSharedStateType(
                    fields[3].to_string(),
                ));
            }
        };

        Ok(SharedStateMessage::new(from, to, shared_state_type))
    }
}
impl SharedStateMessage {
    fn new(from: impl AsRef<str>, to: impl AsRef<str>, shared_state_type: SharedStateType) -> Self {
        SharedStateMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            shared_state_type,
        }
    }
    pub fn version(from: impl AsRef<str>, to: impl AsRef<str>) -> SharedStateMessage {
        SharedStateMessage::new(from, to, SharedStateType::Version)
    }
    pub fn id(from: impl AsRef<str>, to: impl AsRef<str>) -> SharedStateMessage {
        SharedStateMessage::new(from, to, SharedStateType::ID)
    }
    pub fn di(from: impl AsRef<str>, to: impl AsRef<str>) -> SharedStateMessage {
        SharedStateMessage::new(from, to, SharedStateType::DI)
    }
    pub fn i_have(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::IHave {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn scratchpad(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        contents: ScratchPad,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::ScratchPad {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                contents,
            },
        )
    }
    pub fn global_data(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        contents: impl AsRef<str>,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::GlobalData {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                contents: contents.as_ref().to_string(),
            },
        )
    }
    pub fn temp_altitude(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        level: Level,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::TempAltitude {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                level,
            },
        )
    }
    pub fn beacon_code(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        code: TransponderCode,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::BeaconCode {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                code,
            },
        )
    }
    pub fn voice_type(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        voice_capability: VoiceCapability,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::VoiceType {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                voice_capability,
            },
        )
    }
    pub fn handoff_cancel(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::HandoffCancel {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn point_out(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::PointOut {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn push_to_departure_list(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::PushToDepartureList {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
            },
        )
    }
    pub fn flight_strip(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        aircraft_callsign: impl AsRef<str>,
        format: Option<i32>,
        contents: Option<Vec<String>>,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::FlightStrip {
                aircraft_callsign: aircraft_callsign.as_ref().to_uppercase(),
                format,
                contents,
            },
        )
    }
    pub fn land_line(
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        landline_type: LandLineType,
        landline_command: LandLineCommand,
    ) -> SharedStateMessage {
        SharedStateMessage::new(
            from,
            to,
            SharedStateType::LandLine {
                landline_type,
                landline_command,
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct HandoffAcceptMessage {
    pub from: String,
    pub to: String,
    pub aircraft: String,
}

impl Display for HandoffAcceptMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$HA{}:{}:{}", self.from, self.to, self.aircraft)
    }
}

impl TryFrom<&[&str]> for HandoffAcceptMessage {
    type Error = FsdMessageParseError;
    fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
        if fields.len() < 3 {
            return Err(FsdMessageParseError::InvalidFieldCount(3, fields.len()));
        }
        let first = &fields[0][3..];
        Ok(HandoffAcceptMessage::new(first, fields[1], fields[2]))
    }
}

impl HandoffAcceptMessage {
    pub fn new(from: impl AsRef<str>, to: impl AsRef<str>, aircraft: impl AsRef<str>) -> Self {
        HandoffAcceptMessage {
            from: from.as_ref().to_uppercase(),
            to: to.as_ref().to_uppercase(),
            aircraft: aircraft.as_ref().to_uppercase(),
        }
    }
}
