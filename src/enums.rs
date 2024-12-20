use std::collections::HashSet;
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

///
#[derive(Debug, Clone, Copy)]
pub enum SimulatorType {
    MSFS95 = 1,
    MSFS98,
    MSCFS,
    MSFS2000,
    MSCFS2,
    MSFS2002,
    MSCFS3,
    MSFS2004,
    MSFSX,
    XPlane8 = 12,
    XPlane9,
    XPlane10,
    XPlane11 = 16,
    FlightGear = 25,
    P3D = 30,
}

impl FromStr for SimulatorType {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(SimulatorType::MSFS95),
            "2" => Ok(SimulatorType::MSFS98),
            "3" => Ok(SimulatorType::MSCFS),
            "4" => Ok(SimulatorType::MSFS2000),
            "5" => Ok(SimulatorType::MSCFS2),
            "6" => Ok(SimulatorType::MSFS2002),
            "7" => Ok(SimulatorType::MSCFS3),
            "8" => Ok(SimulatorType::MSFS2004),
            "9" => Ok(SimulatorType::MSFSX),
            "12" => Ok(SimulatorType::XPlane8),
            "13" => Ok(SimulatorType::XPlane9),
            "14" => Ok(SimulatorType::XPlane10),
            "16" => Ok(SimulatorType::XPlane11),
            "25" => Ok(SimulatorType::FlightGear),
            "30" => Ok(SimulatorType::P3D),
            _ => Err(FsdMessageParseError::InvalidSimulatorType(s.to_string())),
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
    FSInnPlaneInformationRequestMessage,
    FSInnPlaneInformationResponseMessage,
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
            return Err(FsdMessageParseError::UnknownMessageType(
                message.to_string(),
            ));
        }

        if fields[0].starts_with("#AA") {
            return Ok(Self::AtcRegisterMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#AP") {
            return Ok(Self::PilotRegisterMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#DA") {
            return Ok(Self::AtcDeregisterMessage(fields.as_slice().try_into()?));
        }
        if fields[0].starts_with("#DP") {
            return Ok(Self::PilotDeregisterMessage(fields.as_slice().try_into()?));
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
    IsValidATC(String),                             //ATC
    Capabilities,                                   //CAPS
    Com1Freq,                                       //C?
    RealName,                                       //RN
    Server,                                         //SV
    ATIS,                                           //ATIS
    PublicIP,                                       //IP
    INF,                                            //INF
    FlightPlan(String),                             //FP
    ForceBeaconCode(TransponderCode),               //IPC:W:852
    RequestRelief,                                  //BY
    CancelRequestRelief,                            //HI
    HelpRequest(Option<String>),                    //HLP
    CancelHelpRequest(Option<String>),              //NOHLP
    WhoHas(String),                                 //WH
    InitiateTrack(String),                          //IT
    AcceptHandoff(String, String),                  //HT
    DropTrack(String),                              //DR
    SetFinalAltitude(String, u32),                  //FA
    SetTempAltitude(String, u32),                   //TA
    SetBeaconCode(String, TransponderCode),         //BC
    SetScratchpad(String, ScratchPad),              //SC
    SetVoiceType(String, VoiceCapability),          //VT
    AircraftConfigurationRequest,                   //ACC
    AircraftConfigurationResponse(AircraftConfig),  //ACC
    Simtime(DateTime<Utc>),                         //SIMTIME
    //NewInfo,                                      //NEWINFO
    NewATIS(char, String, String),                  //NEWATIS
                                                    //Estimate, //EST
                                                    //SetGlobalData, //GD
}

impl Display for ClientQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientQueryType::IsValidATC(subject) => write!(f, "ATC:{}", subject),
            ClientQueryType::Capabilities => write!(f, "CAPS"),
            ClientQueryType::Com1Freq => write!(f, "C?"),
            ClientQueryType::RealName => write!(f, "RN"),
            ClientQueryType::Server => write!(f, "SV"),
            ClientQueryType::ATIS => write!(f, "ATIS"),
            ClientQueryType::PublicIP => write!(f, "IP"),
            ClientQueryType::INF => write!(f, "INF"),
            ClientQueryType::FlightPlan(subject) => write!(f, "FP:{}", subject),
            ClientQueryType::RequestRelief => write!(f, "BY"),
            ClientQueryType::CancelRequestRelief => write!(f, "HI"),
            ClientQueryType::HelpRequest(None) => write!(f, "HLP"),
            ClientQueryType::HelpRequest(Some(msg)) => write!(f, "HLP:{msg}"),
            ClientQueryType::CancelHelpRequest(None) => write!(f, "NOHLP"),
            ClientQueryType::CancelHelpRequest(Some(msg)) => write!(f, "NOHLP:{msg}"),
            ClientQueryType::WhoHas(subject) => write!(f, "WH:{}", subject),
            ClientQueryType::InitiateTrack(subject) => write!(f, "IT:{}", subject),
            ClientQueryType::AcceptHandoff(subject_ac, subject_atc) => {
                write!(f, "HT:{}:{}", subject_ac, subject_atc)
            }
            ClientQueryType::DropTrack(subject) => write!(f, "DR:{}", subject),
            ClientQueryType::SetFinalAltitude(subject, alt) => write!(f, "FA:{}:{}", subject, alt),
            ClientQueryType::SetTempAltitude(subject, alt) => write!(f, "TA:{}:{}", subject, alt),
            ClientQueryType::SetBeaconCode(subject, code) => {
                write!(f, "BC:{}:{}", subject, code)
            }
            ClientQueryType::ForceBeaconCode(code) => {
                write!(f, "IPC:W:852:{}", code.as_bcd_format())
            }
            ClientQueryType::SetScratchpad(subject, contents) => {
                write!(f, "SC:{}:{}", subject, contents)
            }
            ClientQueryType::SetVoiceType(subject, voice_type) => {
                write!(f, "VT:{}:{}", subject, voice_type)
            }
            ClientQueryType::AircraftConfigurationRequest => {
                write!(f, "ACC:{{\"request\":\"full\"}}")
            }
            ClientQueryType::AircraftConfigurationResponse(aircraft_config) => {
                write!(f, "ACC:{}", aircraft_config)
            }
            ClientQueryType::NewATIS(letter, wind, pressure) => {
                write!(f, "NEWATIS:ATIS {}:  {} - {}", letter, wind, pressure)
            }
            ClientQueryType::Simtime(time) => {
                write!(f, "SIMTIME:{}", time.format("Y%m%d%H%M%S"))
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
    Com1Freq(RadioFrequency),
    ATIS(AtisLine),
    RealName(String, String, u8),
    Capabilities(HashSet<ClientCapability>),
    PublicIP(String),
    Server(String),
    IsValidATC(String, bool),
}
impl Display for ClientResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientResponseType::Com1Freq(frequency) => {
                write!(f, "C?:{}", frequency.to_human_readable_string())
            }
            ClientResponseType::ATIS(atis_line) => write!(f, "ATIS:{}", atis_line),
            ClientResponseType::RealName(name, info, rating) => {
                write!(f, "RN:{}:{}:{}", name, info, rating)
            }
            ClientResponseType::Capabilities(capabilities) => {
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
            ClientResponseType::PublicIP(ip) => write!(f, "IP:{}", ip),
            ClientResponseType::Server(server) => write!(f, "SV:{}", server),
            ClientResponseType::IsValidATC(subject, valid) => {
                let valid = if *valid { 'Y' } else { 'N' };
                write!(f, "ATC:{}:{}", valid, subject)
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
    IHave(String),
    ScratchPad(String, ScratchPad),
    TempAltitude(String, u32),
    FinalAltitude(String, u32),
    VoiceType(String, VoiceCapability),
    BeaconCode(String, TransponderCode),
    HandoffCancel(String),
}

impl Display for SharedStateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SharedStateType::Version => write!(f, "VER"),
            SharedStateType::ID => write!(f, "ID"),
            SharedStateType::DI => write!(f, "DI"),
            SharedStateType::IHave(subject) => write!(f, "IH:{}", subject),
            SharedStateType::ScratchPad(subject, contents) => {
                write!(f, "SC:{}:{}", subject, contents)
            }
            SharedStateType::TempAltitude(subject, altitude) => {
                write!(f, "TA:{}:{}", subject, altitude)
            }
            SharedStateType::FinalAltitude(subject, altitude) => {
                write!(f, "FA:{}:{}", subject, altitude)
            }
            SharedStateType::VoiceType(subject, voice_type) => {
                write!(f, "VT:{}:{}", subject, voice_type)
            }
            SharedStateType::BeaconCode(subject, code) => write!(f, "BC:{}:{}", subject, code),
            SharedStateType::HandoffCancel(subject) => write!(f, "HC:{}", subject),
        }
    }
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
