//! # FSD Messages
//!
//! ## What is FSD?
//!
//! The FSD (Flight Simulator Daemon) protocol is used for communication between pilot / ATC
//! client software and FSD servers via a TCP connection. It has existed since the 90s and
//! is still in wide use today.
//! /
//! The earliest version of an FSD server available online is [this one here](https://github.com/kuroneko/fsd),
//! written by Marty Bochane. It is open source and some people compile and run an instance of
//! this software privately - often virtual flying groups.
//!
//! [VATSIM](https://www.vatsim.net/) and [IVAO](https://www.ivao.aero/) also use the FSD protocol, however they have each diverged significantly from the version of
//! the protocol used in Marty Bochane's server, in such a way that all three of these 'dialects' are
//! incompatible with each other. That said, there are some clients, namely [EuroScope](https://www.euroscope.hu/) and [Swift](https://docs.swift-project.org/doku.php?id=start)
//! which implement both the legacy FSD protocol and the modern VATSIM version, and as such are able to connect both to VATSIM
//! and to private FSD servers.
//!
//! ## What does the FSD protocol look like?
//!
//! Each message starts with a prefix which identifies the type of message, and this is followed by a variable number of colon-delimited fields. For example:
//!
//! `$CQEHAM_GND:@94835:WH:KLM167`
//!
//! `&CQ` - this means the message is a client query.
//!
//! `EHAM_GND` - the callsign of the station sending the message.
//!
//! `@94835` - this is actually how radio frequencies are encoded. This would be 194.835. The astute amongst you may have noticed that this falls way outside of the range
//! used by airband radio - this is a 'special' frequency used by clients to pass information about aircraft.
//!
//! `WH` - this signifies that the message is a 'who has' request - the controller client is sending out a message to all other controller clients in the area to ask if any of them have the
//! aircraft assumed.
//!
//! `KLM123` - this is the aircraft that the controller client is asking about.
//!
//!
//! ## What does this crate do?
//!
//! At the moment, this crate only works with the VATSIM flavour of the FSD protocol. In due course, it will support legacy FSD packets as well as IVAO packets.
//!
//! - Identifies if a string of text is a valid FSD protocol message and identifies the type
//! - Deserialises it into a struct so that you can work with the information in it
//! - Serialises structs into valid, validity-checked FSD message strings
//!
//! ## Examples
//! ```
//! // Imagine this is a message we have received from an FSD server
//! let message_text = String::from("$CQEHAM_GND:@94835:WH:KLM167");
//!
//! // We can identify what type of message it is, deserialise it
//! let message_deserialised = fsd_interface::parse_message(&message_text).unwrap();
//! if let fsd_interface::FsdMessageType::ClientQueryMessage(client_query_message) = message_deserialised {
//!
//!     // And access its data
//!     assert_eq!("EHAM_GND", client_query_message.from.as_str());
//!     assert_eq!("@94835", client_query_message.to.as_str());
//!     if let fsd_interface::ClientQueryType::WhoHas { aircraft_callsign } = client_query_message.query_type {
//!         assert_eq!("KLM167", aircraft_callsign.as_str());
//!     }
//!
//!     // Plus, on the flip side, we can create our own messages and serialise them
//!     let new_message = fsd_interface::messages::ClientQueryMessage::who_has("LIRF_TWR", "@94835", "ITY1561");
//!     assert_eq!(String::from("$CQLIRF_TWR:@94835:WH:ITY1561"), new_message.to_string());
//! }
//! ```
//!
//!
//! ## Disclaimer
//!
//! It is against the VATSIM [Code of Conduct](https://vatsim.net/docs/policy/code-of-conduct) and
//! [User Agreement](https://cdn.vatsim.net/policy-documents/User_Agreement_v1.2.pdf) to attempt to connect to a VATSIM server
//! with client software that has not been approved for use.
//!
//! Of course, you're well within your rights to use this crate to write a client that connects to a private FSD server.
//!
//! If you _do_ obtain permission from VATSIM to connect with your own client software and decide to use this crate, you are
//! responsible for checking that it is indeed compliant with the VATSIM FSD protocol.

#![allow(clippy::too_many_arguments)]

mod aircraft_config;
mod enums;

/// Contains error types used in the crate
pub mod errors;

pub mod messages;
mod structs;
mod util;

pub use aircraft_config::*;
pub use chrono::{DateTime, Utc};
pub use enums::*;
pub use structs::*;

/// Deserialises a valid FSD message string into a struct.
///
/// If the string is a valid FSD message, deserialises it into the appropriate struct and returns it inside an [`FsdMessageType`] enum variant that indicates which type it is.
/// If there are any validation errors, an [`FsdMessageParseError`][errors::FsdMessageParseError] is returned instead.
pub fn parse_message(
    message: impl AsRef<str>,
) -> Result<FsdMessageType, errors::FsdMessageParseError> {
    FsdMessageType::identify(message.as_ref())
}
