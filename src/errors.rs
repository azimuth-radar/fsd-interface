use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsdMessageParseError {
    #[error("invalid field count. Expected {0}, found {1}.")]
    InvalidFieldCount(usize, usize),
    #[error("{0} is not a valid rating")]
    InvalidRating(String),
    #[error("{0} is not a valid protocol revision")]
    InvalidProtocolRevison(String),
    #[error("{0} is not a valid flight rules")]
    InvalidFlightRules(String),
    #[error("{0} is not a valid simulator type")]
    InvalidSimulatorType(String),
    #[error("{0} is not a valid ATC type")]
    InvalidAtcType(String),
    #[error("{0} is not a valid time")]
    InvalidTime(String),
    #[error("{0} is not a valid minute")]
    InvalidMinute(String),
    #[error("{0} is not a valid index")]
    InvalidIndex(String),
    #[error("{0} is not a valid ATC frequency")]
    InvalidFrequency(String),
    #[error("{0} is not a valid visibility range")]
    InvalidVisRange(String),
    #[error("{0} is not a valid lat / long coordinate")]
    InvalidCoordinate(String),
    #[error("{0} is not a transponder mode")]
    InvalidTransponderMode(String),
    #[error("{0} is not a transponder code")]
    InvalidTransponderCode(String),
    #[error("Unable to parse aircraft config: {0}")]
    InvalidAircraftConfig(String),
    #[error("{0} is not a valid pitch / bank / heading number")]
    InvalidPitchBankHeading(String),
    #[error("{0} is not a valid level")]
    InvalidLevel(String),
    #[error("{0} is not a valid altitude")]
    InvalidAltitude(String),
    #[error("{0} is not a valid altitude difference")]
    InvalidAltitudeDifference(String),
    #[error("{0} is not a valid voice capability")]
    InvalidVoiceCapability(String),
    #[error("{0} is not a valid speed")]
    InvalidSpeed(String),
    #[error("{0} is not a valid client ID")]
    InvalidClientID(String),
    #[error("{0} is not a valid version number part")]
    InvalidVersionNumber(String),
    #[error("{0} is not a valid nosewheel angle")]
    InvalidNosewheelAngle(String),
    #[error("{0} is not a valid position velocity")]
    InvalidPositionVelocity(String),
    #[error("Unknown message type: {0}")]
    UnknownMessageType(String),
    #[error("{0} is not a valid ping time")]
    InvalidPingTime(String),
    #[error("{0} is not a valid server error")]
    InvalidServerError(String),
    #[error("{0} is not a valid client query type")]
    InvalidClientQueryType(String),
    #[error("{0} is not a valid new ATIS message")]
    InvalidNewAtisMessage(String),
    #[error("{0} is not a valid valid ATC status")]
    InvalidValidAtcStatus(String),
    #[error("{0} is not a valid valid ATIS line")]
    InvalidATISLine(String),
    #[error("{0} is not a valid valid shared state type")]
    InvalidSharedStateType(String),
    #[error("{0} is not a valid client capability")]
    InvalidClientCapability(String),
    #[error("{0} is not a valid IP addrees")]
    InvalidIPAddress(String),
    #[error("{0} is not a valid port")]
    InvalidPort(String),
}

/// An error message received from the FSD server
#[derive(Debug, Clone, Error)]
pub enum FsdError {
    /// Attempted to log in with a callsign that is already in use
    #[error("Callsign in use")]
    CallsignInUse,
    /// Attempted to log in with an invalid callsign
    #[error("Invalid callsign")]
    InvalidCallsign,
    /// Client attempted to register itself more than once on server
    #[error("Already registered")]
    AlreadyRegistered,
    /// Packet with invalid syntax sent to the FSD server
    #[error("Syntax error")]
    SyntaxError,
    /// Client attempted to send a server message with the wrong 'from' callsign
    #[error("Invalid source callsign")]
    InvalidSourceCallsign,
    /// Attempted to log into the server with an invalid CID or password
    #[error("Invalid CID / password")]
    InvalidCidPassword,
    /// Attempted to perform an action (e.g. kill, flight plan request) on an invalid callsign
    #[error("No such callsign as {0}")]
    NoSuchCallsign(String),
    /// Attempted to retrieve a flight plan but no flight plan exists for that callsign
    #[error("No flight plan for {0}")]
    NoFlightPlan(String),
    /// Requested a METAR or weather information but no weather profile was found for this location
    #[error("No weather profile for {0}")]
    NoWeatherProfile(String),
    /// Attempted to connect using a protocol version not supported by the server
    #[error("Invalid protocol revision")]
    InvalidProtocolRevision,
    /// Attempted to log in with too high a rating
    #[error("Requested level too high")]
    RequestedLevelTooHigh,
    /// Server is full
    #[error("Server full")]
    ServerFull,
    /// Attemped to log in with a CID that has been suspended
    #[error("CID has been suspended")]
    CertificateSuspended,
    /// Attempted to perform an unauthorised ATC action (such as assume an aircraft)
    #[error("Invalid control")]
    InvalidControl,
    /// Attempted to log in to an ATC position which the user's rating does not allow
    #[error("Invalid position for rating")]
    InvalidPositionForRating,
    /// Connected with a client that is not approved for use on the network
    #[error("Unauthorised client")]
    UnauthorisedClient,
    /// Client did not respond to server's authentication challenge in time
    #[error("Authentication time out")]
    AuthTimeOut,
    /// Other error
    #[error("Other: {0}")]
    Other(String),
}
impl FsdError {
    pub fn error_number(&self) -> u8 {
        match *self {
            FsdError::CallsignInUse => 1,     // Fatal
            FsdError::InvalidCallsign => 2,   // Fatal
            FsdError::AlreadyRegistered => 3, // Fatal
            FsdError::SyntaxError => 4,
            FsdError::InvalidSourceCallsign => 5,
            FsdError::InvalidCidPassword => 6, // Fatal
            FsdError::NoSuchCallsign(_) => 7,
            FsdError::NoFlightPlan(_) => 8,
            FsdError::NoWeatherProfile(_) => 9,
            FsdError::InvalidProtocolRevision => 10, // Fatal
            FsdError::RequestedLevelTooHigh => 11,   // Fatal
            FsdError::ServerFull => 12,              // Fatal
            FsdError::CertificateSuspended => 13,    // Fatal
            FsdError::InvalidControl => 14,
            FsdError::InvalidPositionForRating => 15, // Fatal
            FsdError::UnauthorisedClient => 16,       // Fatal
            FsdError::AuthTimeOut => 17,              // Fatal
            FsdError::Other(_) => 18,
        }
    }
    pub fn is_fatal(&self) -> bool {
        const FATAL_ERRORS: [u8; 11] = [1, 2, 3, 6, 10, 11, 12, 13, 15, 16, 17];
        FATAL_ERRORS.contains(&self.error_number())
    }
}
