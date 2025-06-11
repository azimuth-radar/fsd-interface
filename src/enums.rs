use std::net::Ipv4Addr;
use std::{fmt::Display, str::FromStr};

use crate::messages::*;
use crate::structs::{RadioFrequency, TransponderCode};
use crate::{aircraft_config::AircraftConfig, errors::FsdMessageParseError};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientCapability {
    Version,
    ATCInfo,
    ModelDesc,
    ACConfig,
    VisUpdate,
    RadarUpdate,
    ATCMulti,
    SecPos,
    IcaoEq,
    FastPos,
    OngoingCoord,
    InterimPos,
    Stealth,
    Teamspeak,
    NewATIS,
    Mumble,
    GlobalData,
    Simulated,
    ObsPilot,
}
impl FromStr for ClientCapability {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "VERSION" => Ok(ClientCapability::Version),
            "ATCINFO" => Ok(ClientCapability::ATCInfo),
            "MODELDESC" => Ok(ClientCapability::ModelDesc),
            "ACCONFIG" => Ok(ClientCapability::ACConfig),
            "VISUPDATE" => Ok(ClientCapability::VisUpdate),
            "RADARUPDATE" => Ok(ClientCapability::RadarUpdate),
            "ATCMULTI" => Ok(ClientCapability::ATCMulti),
            "SECPOS" => Ok(ClientCapability::SecPos),
            "ICAOEQ" => Ok(ClientCapability::IcaoEq),
            "FASTPOS" => Ok(ClientCapability::FastPos),
            "ONGOINGCOORD" => Ok(ClientCapability::OngoingCoord),
            "INTERIMPOS" => Ok(ClientCapability::InterimPos),
            "STEALTH" => Ok(ClientCapability::Stealth),
            "TEAMSPEAK" => Ok(ClientCapability::Teamspeak),
            "NEWATIS" => Ok(ClientCapability::NewATIS),
            "MUMBLE" => Ok(ClientCapability::Mumble),
            "GLOBALDATA" => Ok(ClientCapability::GlobalData),
            "SIMULATED" => Ok(ClientCapability::Simulated),
            "OBSPILOT" => Ok(ClientCapability::ObsPilot),
            _ => Err(FsdMessageParseError::InvalidClientCapability(s.to_string())),
        }
    }
}
impl Display for ClientCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ClientCapability::ACConfig => write!(f, "ACCONFIG"),
            ClientCapability::ATCInfo => write!(f, "ATCINFO"),
            ClientCapability::ModelDesc => write!(f, "MODELDESC"),
            ClientCapability::Version => write!(f, "VERSION"),
            ClientCapability::VisUpdate => write!(f, "VISUPDATE"),
            ClientCapability::RadarUpdate => write!(f, "RADARUPDATE"),
            ClientCapability::ATCMulti => write!(f, "ATCMULTI"),
            ClientCapability::SecPos => write!(f, "SECPOS"),
            ClientCapability::IcaoEq => write!(f, "ICAOEQ"),
            ClientCapability::FastPos => write!(f, "FASTPOS"),
            ClientCapability::OngoingCoord => write!(f, "ONGOINGCOORD"),
            ClientCapability::InterimPos => write!(f, "INTERIMPOS"),
            ClientCapability::Stealth => write!(f, "STEALTH"),
            ClientCapability::Teamspeak => write!(f, "TEAMSPEAK"),
            ClientCapability::NewATIS => write!(f, "NEWATIS"),
            ClientCapability::Mumble => write!(f, "MUMBLE"),
            ClientCapability::GlobalData => write!(f, "GLOBALDATA"),
            ClientCapability::Simulated => write!(f, "SIMULATED"),
            ClientCapability::ObsPilot => write!(f, "OBSPILOT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtcRating {
    Observer = 1,
    S1,
    S2,
    S3,
    C1,
    C2,
    C3,
    I1,
    I2,
    I3,
    Supervisor,
    Administrator,
}
impl FromStr for AtcRating {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rating_u8 =
            u8::from_str(s).map_err(|_| FsdMessageParseError::InvalidRating(s.to_string()))?;
        match rating_u8 {
            1 => Ok(AtcRating::Observer),
            2 => Ok(AtcRating::S1),
            3 => Ok(AtcRating::S2),
            4 => Ok(AtcRating::S3),
            5 => Ok(AtcRating::C1),
            6 => Ok(AtcRating::C2),
            7 => Ok(AtcRating::C3),
            8 => Ok(AtcRating::I1),
            9 => Ok(AtcRating::I2),
            10 => Ok(AtcRating::I3),
            11 => Ok(AtcRating::Supervisor),
            12 => Ok(AtcRating::Administrator),
            _ => Err(FsdMessageParseError::InvalidRating(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PilotRating {
    Student = 1,
    VFR,
    IFR,
    Instructor,
    Supervisor,
}

impl FromStr for PilotRating {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rating_u8 =
            u8::from_str(s).map_err(|_| FsdMessageParseError::InvalidRating(s.to_string()))?;
        match rating_u8 {
            1 => Ok(PilotRating::Student),
            2 => Ok(PilotRating::VFR),
            3 => Ok(PilotRating::IFR),
            4 => Ok(PilotRating::Instructor),
            5 => Ok(PilotRating::Supervisor),
            _ => Err(FsdMessageParseError::InvalidRating(s.to_string())),
        }
    }
}

/// Represents a version of the FSD protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolRevision {
    /// Used on legacy FSD servers. If the FSD server is a privately run one, it is most likely using this version
    Classic = 9,
    /// Deprecated - used on VATSIM prior to the introduction of client authentication
    VatsimNoAuth = 10,
    /// Used on VATSIM servers until 2022
    VatsimAuth = 100,
    /// VATSIM Velocity - used on VATSIM servers since 2022
    Vatsim2022 = 101,
}
impl FromStr for ProtocolRevision {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "9" => Ok(Self::Classic),
            "10" => Ok(Self::VatsimNoAuth),
            "100" => Ok(Self::VatsimAuth),
            "101" => Ok(Self::Vatsim2022),
            _ => Err(FsdMessageParseError::InvalidProtocolRevison(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SimulatorType {
    Unknown,
    MSFS95,
    MSFS98,
    MSCFS,
    MSFS2000,
    MSCFS2,
    MSFS2002,
    MSCFS3,
    MSFS2004,
    MSFSX,
    MSFS,
    MSFS2024,
    XPLANE8,
    XPLANE9,
    XPLANE10,
    XPLANE11,
    XPLANE12,
    P3Dv1,
    P3Dv2,
    P3Dv3,
    P3Dv4,
    P3Dv5,
    FlightGear,
}
impl<S: AsRef<str>> From<S> for SimulatorType {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "1" => SimulatorType::MSFS95,
            "2" => SimulatorType::MSFS98,
            "3" => SimulatorType::MSCFS,
            "4" => SimulatorType::MSFS2000,
            "5" => SimulatorType::MSCFS2,
            "6" => SimulatorType::MSFS2002,
            "7" => SimulatorType::MSCFS3,
            "8" => SimulatorType::MSFS2004,
            "9" => SimulatorType::MSFSX,
            "10" => SimulatorType::MSFS,
            "11" => SimulatorType::MSFS2024,
            "12" => SimulatorType::XPLANE8,
            "13" => SimulatorType::XPLANE9,
            "14" => SimulatorType::XPLANE10,
            "15" => SimulatorType::XPLANE11,
            "16" => SimulatorType::XPLANE12,
            "17" => SimulatorType::P3Dv1,
            "18" => SimulatorType::P3Dv2,
            "19" => SimulatorType::P3Dv3,
            "20" => SimulatorType::P3Dv4,
            "21" => SimulatorType::P3Dv5,
            "22" => SimulatorType::FlightGear,
            _ => SimulatorType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightRules {
    DVFR,
    SVFR,
    VFR,
    IFR,
}

impl FromStr for FlightRules {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        match s.as_str() {
            "D" => Ok(FlightRules::DVFR),
            "S" => Ok(FlightRules::SVFR),
            "V" => Ok(FlightRules::VFR),
            "I" => Ok(FlightRules::IFR),
            _ => Err(FsdMessageParseError::InvalidFlightRules(s)),
        }
    }
}

impl Display for FlightRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FlightRules::DVFR => write!(f, "D"),
            FlightRules::VFR => write!(f, "V"),
            FlightRules::SVFR => write!(f, "S"),
            FlightRules::IFR => write!(f, "I"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtcType {
    Observer,
    FlightServiceStation,
    Delivery,
    Ground,
    Tower,
    Approach,
    Centre,
}

impl FromStr for AtcType {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(AtcType::Observer),
            "1" => Ok(AtcType::FlightServiceStation),
            "2" => Ok(AtcType::Delivery),
            "3" => Ok(AtcType::Ground),
            "4" => Ok(AtcType::Tower),
            "5" => Ok(AtcType::Approach),
            "6" => Ok(AtcType::Centre),
            _ => Err(FsdMessageParseError::InvalidAtcType(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransponderMode {
    Standby,
    ModeC,
    Ident,
}
impl FromStr for TransponderMode {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(TransponderMode::Standby),
            "N" => Ok(TransponderMode::ModeC),
            "Y" => Ok(TransponderMode::Ident),
            _ => Err(FsdMessageParseError::InvalidTransponderMode(s.to_string())),
        }
    }
}
impl Display for TransponderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Standby => write!(f, "S"),
            Self::ModeC => write!(f, "N"),
            Self::Ident => write!(f, "Y"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FsdMessageType {
    AtcRegisterMessage(AtcRegisterMessage),
    PilotRegisterMessage(PilotRegisterMessage),
    AtcDeregisterMessage(AtcDeregisterMessage),
    PilotDeregisterMessage(PilotDeregisterMessage),
    AtcPositionUpdateMessage(AtcPositionUpdateMessage),
    AtcSecondaryVisCentreMessage(AtcSecondaryVisCentreMessage),
    PilotPositionUpdateMessage(PilotPositionUpdateMessage),
    AuthenticationChallengeMessage(AuthenticationChallengeMessage),
    AuthenticationResponseMessage(AuthenticationResponseMessage),
    TextMessage(TextMessage),
    FrequencyMessage(FrequencyMessage),
    ChangeServerMessage(ChangeServerMessage),
    InitialServerHandshakeMessage(InitialServerHandshakeMessage),
    InitialClientHandshakeMessage(InitialClientHandshakeMessage),
    SendFastPositionUpdatesMessage(SendFastPositionUpdatesMessage),
    VelocityPositionStoppedMessage(VelocityPositionStoppedMessage),
    VelocityPositionSlowMessage(VelocityPositionSlowMessage),
    VelocityPositionFastMessage(VelocityPositionFastMessage),
    KillMessage(KillMessage),
    MetarRequestMessage(MetarRequestMessage),
    MetarResponseMessage(MetarResponseMessage),
    PingMessage(PingMessage),
    PongMessage(PongMessage),
    PlaneInfoRequestMessage(PlaneInfoRequestMessage),
    PlaneInfoResponseMessage(PlaneInfoResponseMessage),
    FsdErrorMessage(FsdErrorMessage),
    FlightPlanMessage(FlightPlanMessage),
    FlightPlanAmendmentMessage(FlightPlanAmendmentMessage),
    FSInnPlaneInformationRequestMessage,  // Deprecated
    FSInnPlaneInformationResponseMessage, // Deprecated
    ServerHeartbeat,
    ClientQueryMessage(ClientQueryMessage),
    ClientQueryResponseMessage(ClientQueryResponseMessage),
    HandoffOfferMessage(HandoffOfferMessage),
    HandoffAcceptMessage(HandoffAcceptMessage),
    SharedStateMessage(SharedStateMessage),
}

impl FsdMessageType {
    pub(crate) fn identify(message: &str) -> Result<FsdMessageType, FsdMessageParseError> {
        let fields: Vec<&str> = message.split(':').collect();
        if fields.len() < 2 {
            if fields[0].starts_with("#DA") {
                return Ok(Self::AtcDeregisterMessage(fields.as_slice().try_into()?));
            }
            if fields[0].starts_with("#DP") {
                return Ok(Self::PilotDeregisterMessage(fields.as_slice().try_into()?));
            }
            return Err(FsdMessageParseError::UnknownMessageType(
                message.to_string(),
            ));
        }
        if fields[0].starts_with("#DA") {
            return Ok(Self::AtcDeregisterMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#DP") {
            return Ok(Self::PilotDeregisterMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#AA") {
            return Ok(Self::AtcRegisterMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#AP") {
            return Ok(Self::PilotRegisterMessage(fields.as_slice().try_into()?));
        }

        if fields[0].starts_with('%') {
            return Ok(Self::AtcPositionUpdateMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with('\'') {
            return Ok(Self::AtcSecondaryVisCentreMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with('@') {
            return Ok(Self::PilotPositionUpdateMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$ZC") {
            return Ok(Self::AuthenticationChallengeMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$ZR") {
            return Ok(Self::AuthenticationResponseMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$ER") {
            return Ok(Self::FsdErrorMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$HO") {
            return Ok(Self::HandoffOfferMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$HA") {
            return Ok(Self::HandoffAcceptMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#TM") {
            if fields[1].starts_with('@') {
                return Ok(Self::FrequencyMessage(fields.as_slice().try_into()?));
            }

            return Ok(Self::TextMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$XX") {
            return Ok(Self::ChangeServerMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$FP") {
            return Ok(Self::FlightPlanMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$AM") {
            return Ok(Self::FlightPlanAmendmentMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$DI") {
            return Ok(Self::InitialServerHandshakeMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$ID") {
            return Ok(Self::InitialClientHandshakeMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$SF") {
            return Ok(Self::SendFastPositionUpdatesMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("#ST") {
            return Ok(Self::VelocityPositionStoppedMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("#DL") {
            return Ok(Self::ServerHeartbeat);
        }
        if fields[0].starts_with("#SL") {
            return Ok(Self::VelocityPositionSlowMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("#PC") {
            return Ok(Self::SharedStateMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with('^') {
            return Ok(Self::VelocityPositionFastMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$!!") {
            return Ok(Self::KillMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$AX") {
            return Ok(Self::MetarRequestMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$AR") {
            return Ok(Self::MetarResponseMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$CQ") {
            return Ok(Self::ClientQueryMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$CR") {
            return Ok(Self::ClientQueryResponseMessage(
                fields.as_slice().try_into()?,
            ));
        }
        if fields[0].starts_with("$PI") {
            return Ok(Self::PingMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("$PO") {
            return Ok(Self::PongMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#SB") {
            if fields[2] == "PIR" {
                return Ok(Self::PlaneInfoRequestMessage(fields.as_slice().try_into()?));
            }
            if fields[2] == "PI" {
                return Ok(Self::PlaneInfoResponseMessage(
                    fields.as_slice().try_into()?,
                ));
            }
            if fields[2] == "FSIPI" {
                return Ok(Self::FSInnPlaneInformationResponseMessage);
            }
            if fields[2] == "FSIPIR" {
                return Ok(Self::FSInnPlaneInformationRequestMessage);
            }
        }

        Err(FsdMessageParseError::UnknownMessageType(
            message.to_string(),
        ))
    }
}
impl Display for FsdMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsdMessageType::AtcRegisterMessage(m) => m.fmt(f),
            FsdMessageType::PilotRegisterMessage(m) => m.fmt(f),
            FsdMessageType::AtcDeregisterMessage(m) => m.fmt(f),
            FsdMessageType::PilotDeregisterMessage(m) => m.fmt(f),
            FsdMessageType::AtcPositionUpdateMessage(m) => m.fmt(f),
            FsdMessageType::AtcSecondaryVisCentreMessage(m) => m.fmt(f),
            FsdMessageType::PilotPositionUpdateMessage(m) => m.fmt(f),
            FsdMessageType::AuthenticationChallengeMessage(m) => m.fmt(f),
            FsdMessageType::AuthenticationResponseMessage(m) => m.fmt(f),
            FsdMessageType::TextMessage(m) => m.fmt(f),
            FsdMessageType::FrequencyMessage(m) => m.fmt(f),
            FsdMessageType::ChangeServerMessage(m) => m.fmt(f),
            FsdMessageType::InitialServerHandshakeMessage(m) => m.fmt(f),
            FsdMessageType::InitialClientHandshakeMessage(m) => m.fmt(f),
            FsdMessageType::SendFastPositionUpdatesMessage(m) => m.fmt(f),
            FsdMessageType::VelocityPositionStoppedMessage(m) => m.fmt(f),
            FsdMessageType::VelocityPositionSlowMessage(m) => m.fmt(f),
            FsdMessageType::VelocityPositionFastMessage(m) => m.fmt(f),
            FsdMessageType::KillMessage(m) => m.fmt(f),
            FsdMessageType::MetarRequestMessage(m) => m.fmt(f),
            FsdMessageType::MetarResponseMessage(m) => m.fmt(f),
            FsdMessageType::PingMessage(m) => m.fmt(f),
            FsdMessageType::PongMessage(m) => m.fmt(f),
            FsdMessageType::PlaneInfoRequestMessage(m) => m.fmt(f),
            FsdMessageType::PlaneInfoResponseMessage(m) => m.fmt(f),
            FsdMessageType::FsdErrorMessage(m) => m.fmt(f),
            FsdMessageType::FlightPlanMessage(m) => m.fmt(f),
            FsdMessageType::FlightPlanAmendmentMessage(m) => m.fmt(f),
            m @ FsdMessageType::FSInnPlaneInformationRequestMessage => m.fmt(f),
            m @ FsdMessageType::FSInnPlaneInformationResponseMessage => m.fmt(f),
            m @ FsdMessageType::ServerHeartbeat => m.fmt(f),
            FsdMessageType::ClientQueryMessage(m) => m.fmt(f),
            FsdMessageType::ClientQueryResponseMessage(m) => m.fmt(f),
            FsdMessageType::HandoffOfferMessage(m) => m.fmt(f),
            FsdMessageType::HandoffAcceptMessage(m) => m.fmt(f),
            FsdMessageType::SharedStateMessage(m) => m.fmt(f),
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum ClientQueryType {
    IsValidATC {
        atc_callsign: String,
    }, //ATC
    Capabilities, //CAPS
    Com1Freq,     //C?
    RealName,     //RN
    Server,       //SV
    ATIS,         //ATIS
    PublicIP,     //IP
    INF,          //INF
    FlightPlan {
        aircraft_callsign: String,
    }, //FP
    ForceBeaconCode {
        code: TransponderCode,
    }, //IPC:W:852
    RequestRelief, //BY
    CancelRequestRelief, //HI
    HelpRequest {
        message: Option<String>,
    }, //HLP
    CancelHelpRequest {
        message: Option<String>,
    }, //NOHLP
    WhoHas {
        aircraft_callsign: String,
    }, //WH
    InitiateTrack {
        aircraft_callsign: String,
    }, //IT
    AcceptHandoff {
        aircraft_callsign: String,
        atc_callsign: String,
    }, //HT
    DropTrack {
        aircraft_callsign: String,
    }, //DR
    SetFinalAltitude {
        aircraft_callsign: String,
        level: Level,
    }, //FA
    SetTempAltitude {
        aircraft_callsign: String,
        level: Level,
    }, //TA
    SetBeaconCode {
        aircraft_callsign: String,
        code: TransponderCode,
    }, //BC
    SetScratchpad {
        aircraft_callsign: String,
        contents: ScratchPad,
    }, //SC
    SetVoiceType {
        aircraft_callsign: String,
        voice_capability: VoiceCapability,
    }, //VT
    AircraftConfigurationRequest, //ACC
    AircraftConfigurationResponse {
        aircraft_config: AircraftConfig,
    }, //ACC
    SimTime {
        time: DateTime<Utc>,
    }, //SIMTIME
    NewInfo {
        atis_letter: char,
    }, //NEWINFO
    NewATIS {
        atis_letter: char,
        surface_wind: String,
        pressure: String,
    }, //NEWATIS
    //Estimate,                                                                     //EST
    SetGlobalData {
        aircraft_callsign: String,
        contents: String,
    }, //GD
}

impl Display for ClientQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientQueryType::IsValidATC { atc_callsign } => write!(f, "ATC:{}", atc_callsign),
            ClientQueryType::Capabilities => write!(f, "CAPS"),
            ClientQueryType::Com1Freq => write!(f, "C?"),
            ClientQueryType::RealName => write!(f, "RN"),
            ClientQueryType::Server => write!(f, "SV"),
            ClientQueryType::ATIS => write!(f, "ATIS"),
            ClientQueryType::PublicIP => write!(f, "IP"),
            ClientQueryType::INF => write!(f, "INF"),
            ClientQueryType::FlightPlan { aircraft_callsign } => {
                write!(f, "FP:{}", aircraft_callsign)
            }
            ClientQueryType::RequestRelief => write!(f, "BY"),
            ClientQueryType::CancelRequestRelief => write!(f, "HI"),
            ClientQueryType::HelpRequest { message: None } => write!(f, "HLP"),
            ClientQueryType::HelpRequest { message: Some(msg) } => write!(f, "HLP:{msg}"),
            ClientQueryType::CancelHelpRequest { message: None } => write!(f, "NOHLP"),
            ClientQueryType::CancelHelpRequest { message: Some(msg) } => write!(f, "NOHLP:{msg}"),
            ClientQueryType::WhoHas { aircraft_callsign } => write!(f, "WH:{}", aircraft_callsign),
            ClientQueryType::InitiateTrack { aircraft_callsign } => {
                write!(f, "IT:{}", aircraft_callsign)
            }
            ClientQueryType::AcceptHandoff {
                aircraft_callsign,
                atc_callsign,
            } => {
                write!(f, "HT:{}:{}", aircraft_callsign, atc_callsign)
            }
            ClientQueryType::DropTrack { aircraft_callsign } => {
                write!(f, "DR:{}", aircraft_callsign)
            }
            ClientQueryType::SetFinalAltitude {
                aircraft_callsign,
                level,
            } => write!(f, "FA:{}:{}", aircraft_callsign, level),
            ClientQueryType::SetTempAltitude {
                aircraft_callsign,
                level,
            } => write!(f, "TA:{}:{}", aircraft_callsign, level),
            ClientQueryType::SetBeaconCode {
                aircraft_callsign,
                code,
            } => {
                write!(f, "BC:{}:{}", aircraft_callsign, code)
            }
            ClientQueryType::ForceBeaconCode { code } => {
                write!(f, "IPC:W:852:{}", code.as_bcd_format())
            }
            ClientQueryType::SetScratchpad {
                aircraft_callsign,
                contents,
            } => {
                write!(f, "SC:{}:{}", aircraft_callsign, contents)
            }
            ClientQueryType::SetVoiceType {
                aircraft_callsign,
                voice_capability,
            } => {
                write!(f, "VT:{}:{}", aircraft_callsign, voice_capability)
            }
            ClientQueryType::AircraftConfigurationRequest => {
                write!(f, "ACC:{{\"request\":\"full\"}}")
            }
            ClientQueryType::AircraftConfigurationResponse { aircraft_config } => {
                write!(f, "ACC:{}", aircraft_config)
            }
            ClientQueryType::NewATIS {
                atis_letter,
                surface_wind,
                pressure,
            } => {
                write!(
                    f,
                    "NEWATIS:ATIS {}:  {} - {}",
                    atis_letter, surface_wind, pressure
                )
            }
            ClientQueryType::NewInfo { atis_letter } => {
                write!(f, "NEWINFO:{}", atis_letter)
            }
            ClientQueryType::SimTime { time } => {
                write!(f, "SIMTIME:{}", time.format("Y%m%d%H%M%S"))
            }
            ClientQueryType::SetGlobalData {
                aircraft_callsign,
                contents,
            } => {
                write!(f, "GD:{}:{}", aircraft_callsign, contents)
            }
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum AtisLine {
    VoiceServer(String),
    TextLine(String),
    LogoffTime(Option<u16>),
    EndMarker(usize),
}
impl Display for AtisLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtisLine::VoiceServer(voice_server) => write!(f, "V:{}", voice_server),
            AtisLine::TextLine(text) => write!(f, "T:{}", text),
            AtisLine::LogoffTime(Some(time)) => write!(f, "Z:{:04}z", time),
            AtisLine::LogoffTime(None) => write!(f, "Z:z"),
            AtisLine::EndMarker(num_lines) => write!(f, "E:{}", num_lines),
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum ClientResponseType {
    Com1Freq {
        frequency: RadioFrequency,
    },
    ATIS {
        atis_line: AtisLine,
    },
    RealName {
        name: String,
        sector_file: String,
        rating: u8,
    },
    Capabilities {
        capabilities: Vec<ClientCapability>,
    },
    PublicIP {
        ip_address: String,
    },
    Server {
        hostname_or_ip_address: String,
    },
    IsValidATC {
        atc_callsign: String,
        valid_atc: bool,
    },
}
impl Display for ClientResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientResponseType::Com1Freq { frequency } => {
                write!(f, "C?:{}", frequency.to_human_readable_string())
            }
            ClientResponseType::ATIS { atis_line } => write!(f, "ATIS:{}", atis_line),
            ClientResponseType::RealName {
                name,
                sector_file,
                rating,
            } => {
                write!(f, "RN:{}:{}:{}", name, sector_file, rating)
            }
            ClientResponseType::Capabilities { capabilities } => {
                write!(f, "CAPS:")?;
                let mut capabilities = capabilities.iter().peekable();
                while let Some(capability) = capabilities.next() {
                    write!(f, "{}=1", capability)?;
                    if capabilities.peek().is_some() {
                        write!(f, ":")?;
                    }
                }
                Ok(())
            }
            ClientResponseType::PublicIP { ip_address } => write!(f, "IP:{}", ip_address),
            ClientResponseType::Server {
                hostname_or_ip_address,
            } => write!(f, "SV:{}", hostname_or_ip_address),
            ClientResponseType::IsValidATC {
                atc_callsign,
                valid_atc,
            } => {
                let valid = if *valid_atc { 'Y' } else { 'N' };
                write!(f, "ATC:{}:{}", valid, atc_callsign)
            }
        }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum SharedStateType {
    Version,
    ID,
    DI,
    IHave {
        aircraft_callsign: String,
    },
    ScratchPad {
        aircraft_callsign: String,
        contents: ScratchPad,
    },
    TempAltitude {
        aircraft_callsign: String,
        level: Level,
    },
    FinalAltitude {
        aircraft_callsign: String,
        level: Level,
    },
    VoiceType {
        aircraft_callsign: String,
        voice_capability: VoiceCapability,
    },
    BeaconCode {
        aircraft_callsign: String,
        code: TransponderCode,
    },
    HandoffCancel {
        aircraft_callsign: String,
    },
    FlightStrip {
        aircraft_callsign: String,
        format: Option<i32>,
        contents: Option<Vec<String>>,
    },
    PushToDepartureList {
        aircraft_callsign: String,
    },
    PointOut {
        aircraft_callsign: String,
    },
    LandLine {
        landline_type: LandLineType,
        landline_command: LandLineCommand,
    },
    GlobalData {
        aircraft_callsign: String,
        contents: String,
    },
}

impl Display for SharedStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SharedStateType::Version => write!(f, "VER"),
            SharedStateType::ID => write!(f, "ID"),
            SharedStateType::DI => write!(f, "DI"),
            SharedStateType::IHave { aircraft_callsign } => write!(f, "IH:{}", aircraft_callsign),
            SharedStateType::ScratchPad {
                aircraft_callsign,
                contents,
            } => {
                write!(f, "SC:{}:{}", aircraft_callsign, contents)
            }
            SharedStateType::TempAltitude {
                aircraft_callsign,
                level,
            } => {
                write!(f, "TA:{}:{}", aircraft_callsign, level)
            }
            SharedStateType::FinalAltitude {
                aircraft_callsign,
                level,
            } => {
                write!(f, "FA:{}:{}", aircraft_callsign, level)
            }
            SharedStateType::VoiceType {
                aircraft_callsign,
                voice_capability,
            } => {
                write!(f, "VT:{}:{}", aircraft_callsign, voice_capability)
            }
            SharedStateType::BeaconCode {
                aircraft_callsign,
                code,
            } => write!(f, "BC:{}:{}", aircraft_callsign, code),
            SharedStateType::HandoffCancel { aircraft_callsign } => {
                write!(f, "HC:{}", aircraft_callsign)
            }
            SharedStateType::PointOut { aircraft_callsign } => {
                write!(f, "PT:{}", aircraft_callsign)
            }
            SharedStateType::PushToDepartureList { aircraft_callsign } => {
                write!(f, "DP:{}", aircraft_callsign)
            }
            SharedStateType::FlightStrip {
                aircraft_callsign,
                format,
                contents,
            } => {
                write!(f, "ST:{aircraft_callsign}")?;
                if let Some(format) = *format {
                    write!(f, ":{format}")?;
                }
                if let Some(contents) = contents {
                    for item in contents {
                        write!(f, ":{item}")?;
                    }
                }
                Ok(())
            }
            SharedStateType::LandLine {
                landline_type,
                landline_command,
            } => match (*landline_type, *landline_command) {
                (LandLineType::Intercom, LandLineCommand::Request { ip_address, port }) => {
                    write!(f, "IC:{ip_address}:{port}")
                }
                (LandLineType::Intercom, LandLineCommand::Approve { ip_address, port }) => {
                    write!(f, "IK:{ip_address}:{port}")
                }
                (LandLineType::Intercom, LandLineCommand::Reject) => write!(f, "IB"),
                (LandLineType::Intercom, LandLineCommand::End) => write!(f, "EC"),

                (LandLineType::Override, LandLineCommand::Request { ip_address, port }) => {
                    write!(f, "OV:{ip_address}:{port}")
                }
                (LandLineType::Override, LandLineCommand::Approve { ip_address, port }) => {
                    write!(f, "OK:{ip_address}:{port}")
                }
                (LandLineType::Override, LandLineCommand::Reject) => write!(f, "OB"),
                (LandLineType::Override, LandLineCommand::End) => write!(f, "EO"),

                (LandLineType::Monitor, LandLineCommand::Request { ip_address, port }) => {
                    write!(f, "MN:{ip_address}:{port}")
                }
                (LandLineType::Monitor, LandLineCommand::Approve { ip_address, port }) => {
                    write!(f, "MK:{ip_address}:{port}")
                }
                (LandLineType::Monitor, LandLineCommand::Reject) => write!(f, "MB"),
                (LandLineType::Monitor, LandLineCommand::End) => write!(f, "EM"),
            },
            SharedStateType::GlobalData {
                aircraft_callsign,
                contents,
            } => write!(f, "GD:{aircraft_callsign}:{contents}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LandLineType {
    Intercom,
    Override,
    Monitor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LandLineCommand {
    Request { ip_address: Ipv4Addr, port: u16 },
    Approve { ip_address: Ipv4Addr, port: u16 },
    Reject,
    End,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Exactly,
    OrLess,
    OrGreater,
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Exactly => write!(f, "="),
            Self::OrLess => write!(f, "-"),
            Self::OrGreater => write!(f, "+"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GroundState {
    NoState,
    OnFrequency,
    DeIcing,
    Startup,
    Pushback,
    Taxi,
    LineUp,
    TakeOff,
    TaxiIn,
    OnBlock,
}
impl Display for GroundState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoState => write!(f, "NSTS"),
            Self::OnFrequency => write!(f, "ONFREQ"),
            Self::DeIcing => write!(f, "DE-ICE"),
            Self::Startup => write!(f, "STUP"),
            Self::Pushback => write!(f, "PUSH"),
            Self::Taxi => write!(f, "TAXI"),
            Self::LineUp => write!(f, "LINEUP"),
            Self::TakeOff => write!(f, "DEPA"),
            Self::TaxiIn => write!(f, "TXIN"),
            Self::OnBlock => write!(f, "PARK"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScratchPad {
    PlainTextOrDirect(String),
    RateOfClimbDescent(u32),
    Heading(u32),
    Speed(u32),
    Mach(u32),
    SpeedOperator(Operator),
    RateOfClimbDescentOperator(Operator),
    Stand(String),
    CancelledStand,
    ManualStand(String, String),
    CancelledManualStand,
    ClearanceReceived,
    ClearanceCancelled,
    GroundState(GroundState),
    MissedApproach,
}
impl FromStr for ScratchPad {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // EuroScope scratchpad use to sync clearances
        if let Some(Ok(h)) = s.strip_prefix('H').map(str::parse) {
            return Ok(Self::Heading(h));
        }
        if let Some(Ok(r)) = s.strip_prefix('R').map(str::parse) {
            return Ok(Self::RateOfClimbDescent(r));
        }
        if let Some(Ok(speed)) = s.strip_prefix('S').map(str::parse) {
            return Ok(Self::Speed(speed));
        }
        if let Some(Ok(m)) = s.strip_prefix('M').map(str::parse) {
            return Ok(Self::Mach(m));
        }
        if s.len() > 6 && s.starts_with("GRP/S/") {
            return Ok(Self::Stand(s[6..].to_string()));
        }
        if s.len() > 6 && s.starts_with("GRP/M/") {
            let mut split = s.split('/');
            return Ok(Self::ManualStand(
                split.next().unwrap_or("ZZZZ").to_string(),
                split.next().unwrap_or("").to_string(),
            ));
        }
        match s {
            // ASP/ARC operator syntax from EuroScope TopSky plugin
            "/ASP=/" => Ok(Self::SpeedOperator(Operator::Exactly)),
            "/ASP-/" => Ok(Self::SpeedOperator(Operator::OrLess)),
            "/ASP+/" => Ok(Self::SpeedOperator(Operator::OrGreater)),
            "/ARC=/" => Ok(Self::RateOfClimbDescentOperator(Operator::Exactly)),
            "/ARC-/" => Ok(Self::RateOfClimbDescentOperator(Operator::OrLess)),
            "/ARC+/" => Ok(Self::RateOfClimbDescentOperator(Operator::OrGreater)),
            // Stand assignment cancellations from EuroScope GroundRadar plugin
            "GRP/S/" => Ok(Self::CancelledStand),
            "GRP/M/" => Ok(Self::CancelledManualStand),
            // Ground states, mixed plain EuroScope and GroundRadar plugin
            "NSTS" => Ok(Self::GroundState(GroundState::NoState)),
            // TODO: Why is this a separate state?
            "NOSTATE" => Ok(Self::GroundState(GroundState::NoState)),
            "ONFREQ" => Ok(Self::GroundState(GroundState::OnFrequency)),
            "DE-ICE" => Ok(Self::GroundState(GroundState::DeIcing)),
            "STUP" | "ST-UP" => Ok(Self::GroundState(GroundState::Startup)),
            "PUSH" => Ok(Self::GroundState(GroundState::Pushback)),
            "TAXI" => Ok(Self::GroundState(GroundState::Taxi)),
            "LINEUP" => Ok(Self::GroundState(GroundState::LineUp)),
            "TXIN" => Ok(Self::GroundState(GroundState::TaxiIn)),
            "DEPA" => Ok(Self::GroundState(GroundState::TakeOff)),
            "PARK" => Ok(Self::GroundState(GroundState::OnBlock)),
            "CLEA" => Ok(Self::ClearanceReceived),
            "NOTC" => Ok(Self::ClearanceCancelled),
            "MISAP_" => Ok(Self::MissedApproach),
            text => Ok(ScratchPad::PlainTextOrDirect(text.to_string())),
        }
    }
}
impl Display for ScratchPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RateOfClimbDescent(r) => write!(f, "R{r}"),
            Self::Heading(h) => write!(f, "H{h}"),
            Self::Speed(speed) => write!(f, "S{speed}"),
            Self::Mach(m) => write!(f, "{m}"),
            Self::SpeedOperator(op) => write!(f, "/ASP{op}/"),
            Self::RateOfClimbDescentOperator(op) => write!(f, "/ARC{op}/"),
            Self::PlainTextOrDirect(text) => write!(f, "{text}"),
            Self::Stand(stand) => write!(f, "GRP/S/{stand}"),
            Self::CancelledStand => write!(f, "GRP/S/"),
            Self::ManualStand(icao, stand) => write!(f, "GRP/M/{icao}/{stand}"),
            Self::CancelledManualStand => write!(f, "GRP/M"),
            Self::GroundState(gs) => gs.fmt(f),
            Self::ClearanceReceived => write!(f, "CLEA"),
            Self::ClearanceCancelled => write!(f, "NOTC"),
            Self::MissedApproach => write!(f, "MISAP_"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceCapability {
    Unknown,
    Voice,
    Text,
    Receive,
}
impl FromStr for VoiceCapability {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(VoiceCapability::Unknown);
        }
        let s = s.to_lowercase();
        match s.as_str() {
            "v" => Ok(VoiceCapability::Voice),
            "t" => Ok(VoiceCapability::Text),
            "r" => Ok(VoiceCapability::Receive),
            _ => Err(FsdMessageParseError::InvalidVoiceCapability(s)),
        }
    }
}
impl Display for VoiceCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            VoiceCapability::Unknown => write!(f, ""),
            VoiceCapability::Voice => write!(f, "v"),
            VoiceCapability::Text => write!(f, "t"),
            VoiceCapability::Receive => write!(f, "r"),
        }
    }
}

/// Type used for CFL (TA in FSD) and RFL (FA in FSD) to represent the
/// filed or cleared flight level, altitude or special VFR keyword
///
/// Currently no difference in behaviour for Altitude vs FL, needs testing of other clients
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    // Used for example when filing with the "VFR" keyword
    VFR,
    // Flight Level in feet
    FlightLevel(i32),
    // Altitude in feet
    Altitude(i32),
}
impl FromStr for Level {
    type Err = FsdMessageParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            Ok(Self::FlightLevel(0))
        } else if input == "VFR" {
            Ok(Self::VFR)
        } else {
            let flight_level = input.to_uppercase().starts_with("FL");
            let input_trimmed = if flight_level { &input[2..] } else { input };

            let as_num: i32 = input_trimmed
                .parse()
                .map_err(|_| FsdMessageParseError::InvalidLevel(input.to_string()))?;
            Ok(if flight_level {
                Self::FlightLevel(as_num * 100)
            } else {
                Self::FlightLevel(as_num)
            })
        }
    }
}
impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::VFR => f.write_str("VFR"),
            Level::FlightLevel(fl) => f.write_fmt(format_args!("{fl:.0}")),
            Level::Altitude(a) => f.write_fmt(format_args!("{a:.0}")),
        }
    }
}
