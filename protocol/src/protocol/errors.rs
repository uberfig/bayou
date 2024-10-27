use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FetchErr {
    IsTombstone(String),
    RequestErr(String),
    DeserializationErr(String),
    InvalidUrl(String),
    MissingHeader(String),
    VerifyErr(VerifyRequestErr),
}

impl Display for FetchErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchErr::IsTombstone(x) => write!(f, "IsTombstone: {}", x),
            FetchErr::RequestErr(x) => write!(f, "RequestErr: {}", x),
            FetchErr::DeserializationErr(x) => write!(f, "DeserializationErr: {}", x),
            FetchErr::InvalidUrl(x) => write!(f, "InvalidUrl: {}", x),
            FetchErr::MissingHeader(x) => write!(f, "MissingHeader: {}", x),
            FetchErr::VerifyErr(verify_request_err) => {
                write!(f, "VerifyErr: {}", verify_request_err)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VerifyRequestErr {
    MissingHeader(String),
    InvalidTimestamp,
    SignatureVerificationFailure,
    TooOld,
    UnableToObtainKey,
    InvalidSigner,
    NoDomain,
}

impl std::fmt::Display for VerifyRequestErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyRequestErr::MissingHeader(x) => write!(f, "MissingHeader: {}", x),
            VerifyRequestErr::InvalidTimestamp => write!(f, "InvalidTimestamp"),
            VerifyRequestErr::SignatureVerificationFailure => {
                write!(f, "SignatureVerificationFailure")
            }
            VerifyRequestErr::TooOld => write!(f, "TooOld"),
            VerifyRequestErr::UnableToObtainKey => write!(f, "UnableToObtainKey"),
            VerifyRequestErr::InvalidSigner => write!(f, "InvalidSigner"),
            VerifyRequestErr::NoDomain => write!(f, "NoDomain"),
        }
    }
}
