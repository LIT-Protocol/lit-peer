use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::EcdsaError;

/// The current round of the pre-signature participant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Round {
    /// Round 1
    #[default]
    Round1,
    /// Round 2
    Round2,
    /// Round 3
    Round3,
    /// Round 4
    Round4,
}

impl From<Round> for u8 {
    fn from(value: Round) -> Self {
        match value {
            Round::Round1 => 1,
            Round::Round2 => 2,
            Round::Round3 => 3,
            Round::Round4 => 4,
        }
    }
}

impl TryFrom<u8> for Round {
    type Error = EcdsaError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Round::Round1),
            2 => Ok(Round::Round2),
            3 => Ok(Round::Round3),
            4 => Ok(Round::Round4),
            _ => Err(EcdsaError::InvalidRound(value)),
        }
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Round{}", u8::from(*self))
    }
}

impl FromStr for Round {
    type Err = EcdsaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Round1" => Ok(Round::Round1),
            "Round2" => Ok(Round::Round2),
            "Round3" => Ok(Round::Round3),
            "Round4" => Ok(Round::Round4),
            _ => Err(EcdsaError::InvalidRoundParse(s.to_string())),
        }
    }
}

impl Serialize for Round {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        if s.is_human_readable() {
            self.to_string().serialize(s)
        } else {
            u8::from(*self).serialize(s)
        }
    }
}

impl<'de> Deserialize<'de> for Round {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        if d.is_human_readable() {
            let s = String::deserialize(d)?;
            s.parse::<Self>().map_err(serde::de::Error::custom)
        } else {
            let u = u8::deserialize(d)?;
            Self::try_from(u).map_err(serde::de::Error::custom)
        }
    }
}
