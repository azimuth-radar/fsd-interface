use crate::{enums::ClientCapability, errors::FsdMessageParseError, structs::RadioFrequency};
use std::str::FromStr;

pub fn encode_pitch_bank_heading(pitch: f64, bank: f64, heading: f64, on_ground: bool) -> u32 {
    let mut p = pitch / -360.0;
    if p < 0.0 {
        p += 1.0;
    }
    p *= 1024.0;

    let mut b = bank / -360.0;
    if b < 0.0 {
        b += 1.0;
    }
    b *= 1024.0;

    let h = heading / 360.0 * 1024.0;

    ((p as u32) << 22) | ((b as u32) << 12) | ((h as u32) << 2) | ((on_ground as u32) << 1)
}

pub fn decode_pitch_bank_heading(mut input: u32) -> (f64, f64, f64, bool) {
    input >>= 1;
    let on_ground = (input & 1) == 1;
    input >>= 1;
    let mut heading = (input & 1023) as f64;
    input >>= 10;
    let mut bank = (input & 1023) as f64;
    input >>= 10;
    let mut pitch = input as f64;

    pitch = pitch / 1024.0 * -360.0;
    if pitch > 180.0 {
        pitch -= 360.0;
    } else if pitch <= -180.0 {
        pitch += 360.0;
    }

    bank = bank / 1024.0 * -360.0;
    if bank > 180.0 {
        bank -= 360.0;
    } else if bank <= -180.0 {
        bank += 360.0;
    }

    heading = heading / 1024.0 * 360.0;
    if heading < 0.0 {
        heading += 360.0;
    } else if heading >= 360.0 {
        heading -= 360.0;
    }

    (pitch, bank, heading, on_ground)
}

pub fn split_frequencies(input: &str) -> Vec<RadioFrequency> {
    input
        .split(['&', '@'])
        .filter_map(|x| RadioFrequency::from_str(x).ok())
        .collect()
}

pub(crate) fn group_frequencies_without_symbol(frequencies: &[RadioFrequency]) -> String {
    let mut freqs_string = String::with_capacity(6 * frequencies.len() - 1);
    let mut freqs = frequencies.iter().peekable();
    while let Some(freq) = freqs.next() {
        freqs_string.push_str(&freq.to_string());
        if freqs.peek().is_some() {
            freqs_string.push('&');
        }
    }
    freqs_string
}

pub(crate) fn group_frequencies_with_symbol(frequencies: &[RadioFrequency]) -> String {
    let mut freqs_string = String::with_capacity(6 * frequencies.len() - 1);
    let mut freqs = frequencies.iter().peekable();
    while let Some(freq) = freqs.next() {
        freqs_string.push('@');
        freqs_string.push_str(&freq.to_string());
        if freqs.peek().is_some() {
            freqs_string.push('&');
        }
    }
    freqs_string
}

// $CQEGCC_ATIS:@94835:NEWATIS:ATIS B:  31016KT Q1022
pub(crate) fn parse_new_atis(
    input: &[&str],
) -> Result<(char, String, String), FsdMessageParseError> {
    let first = input[0].to_uppercase();
    let last = input[1].trim().to_uppercase();
    let atis_letter = first
        .chars()
        .last()
        .ok_or_else(|| FsdMessageParseError::InvalidNewAtisMessage(format!("{first}:{last}")))?;
    if (atis_letter as u8) < 65 || (atis_letter as u8) > 90 {
        return Err(FsdMessageParseError::InvalidNewAtisMessage(format!(
            "{first}:{last}"
        )));
    }
    let split = last.split(&[' ', '-']).collect::<Vec<&str>>();

    if split[0].len() < 7 {
        return Err(FsdMessageParseError::InvalidNewAtisMessage(format!(
            "{first}:{last}"
        )));
    };
    let wind = split[0].to_string();

    if split[1].len() < 4 {
        return Err(FsdMessageParseError::InvalidNewAtisMessage(format!(
            "{first}:{last}"
        )));
    };
    let pressure = split[1].to_string();

    Ok((atis_letter, wind, pressure))
}

// $CQESSA_A_ATIS:@94835:NEWATIS:ATIS N:  31016KT - Q986

#[inline]
pub(crate) fn assemble_with_colons(slice: &[&str]) -> String {
    let mut buffer = String::new();
    let mut iter = slice.iter().peekable();
    while let Some(chunk) = iter.next() {
        buffer.push_str(chunk);
        if iter.peek().is_some() {
            buffer.push(':');
        }
    }
    buffer
}

pub fn read_capabilities(caps_str: &[&str]) -> Vec<ClientCapability> {
    let mut capabilities: Vec<ClientCapability> = Vec::with_capacity(caps_str.len() / 2);
    if caps_str.is_empty() {
        return capabilities;
    }

    for entry in caps_str {
        let mut split = entry.split('=');
        let k = match split.next() {
            Some(k) => k,
            None => continue,
        };

        let v = match split.next() {
            Some(v) => v.to_string(),
            None => continue,
        };

        if let Ok(capability) = k.to_uppercase().as_str().parse() {
            if v == "1" {
                capabilities.push(capability);
            }
        }
    }
    capabilities
}
