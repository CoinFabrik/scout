#[cfg(feature = "lint_helper")]
extern crate rustc_driver;
#[cfg(feature = "lint_helper")]
extern crate rustc_errors;
#[cfg(feature = "lint_helper")]
extern crate rustc_lint;
#[cfg(feature = "lint_helper")]
extern crate rustc_span;

pub mod ink_lint_message;
pub mod ink_vulnerability_info;
pub mod soroban_lint_message;

use std::borrow::Cow;
#[cfg(feature = "lint_helper")]
use std::net::{IpAddr, Ipv4Addr};

use ink_lint_message::*;
#[cfg(feature = "lint_helper")]
use rustc_lint::{Lint, LintContext};
#[cfg(feature = "lint_helper")]
use rustc_span::Span;
#[cfg(feature = "lint_helper")]
use scout_audit_clippy_utils::diagnostics::{
    span_lint as span_lint_clippy, span_lint_and_help as span_lint_and_help_clippy,
};
#[cfg(feature = "lint_helper")]
use serde_json::json;
use soroban_lint_message::*;
#[cfg(feature = "lint_helper")]
use soroban_lint_message::*;
use strum::{Display, EnumIter};
#[cfg(feature = "lint_helper")]
use tarpc::{client, context, tokio_serde::formats::Json};

#[cfg(feature = "lint_helper")]
use crate::ink_vulnerability_info::*;
#[cfg(feature = "lint_helper")]
use crate::DetectorSocketClient;

#[derive(Debug, Copy, Clone)]
pub enum BlockChain {
    Ink,
    Soroban,
}
use serde::{Deserialize, Serialize};

/// Available detectors for Soroban
#[derive(Debug, Display, Clone, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum SorobanDetector {
    AvoidCoreMemForget,
    AvoidPanicError,
    AvoidUnsafeBlock,
    DivideBeforeMultiply,
    DosUnboundedOperation,
    InsufficientlyRandomValues,
    OverflowCheck,
    SetContractStorage,
    SorobanVersion,
    UnprotectedUpdateCurrentContractWasm,
    UnsafeExpect,
    UnsafeUnwrap,
}

// Available detectors for Ink
#[derive(Debug, Display, Clone, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum InkDetector {
    AssertViolation,
    AvoidCoreMemForget,
    AvoidFormatString,
    DelegateCall,
    DivideBeforeMultiply,
    DosUnboundedOperation,
    DosUnexpectedRevertWithVector,
    InkVersion,
    InsufficientlyRandomValues,
    IntegerOverflowOrUnderflow,
    IteratorsOverIndexing,
    LazyDelegate,
    PanicError,
    #[strum(serialize = "reentrancy-1")]
    Reentrancy1,
    #[strum(serialize = "reentrancy-2")]
    Reentrancy2,
    SetCodeHash,
    SetContractStorage,
    UnprotectedMappingOperation,
    UnprotectedSelfDestruct,
    UnrestrictedTransferFrom,
    UnsafeExpect,
    UnsafeUnwrap,
    UnusedReturnEnum,
    ZeroOrTestAddress,
}

//#[cfg(feature = "lint_helper")]
#[derive(Debug, Clone)]
pub struct DetectorRawInfo {
    pub vuln_name: &'static str,
    pub severity: &'static str,
    pub vuln_category: &'static str,
    pub vuln_short_msg: &'static str,
    pub vuln_long_msg: &'static str,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectorInfo {
    pub vuln_name: String,
    pub severity: String,
    pub vuln_category: String,
    pub vuln_short_msg: String,
    pub vuln_long_msg: String,
    pub span: String,
    pub help: Option<String>,
}

impl DetectorInfo {
    fn from_raw(raw_info: DetectorRawInfo, span: String, help: Option<&str>) -> Self {
        Self {
            vuln_name: raw_info.vuln_name.to_string(),
            severity: raw_info.severity.to_string(),
            vuln_category: raw_info.vuln_category.to_string(),
            vuln_short_msg: raw_info.vuln_short_msg.to_string(),
            vuln_long_msg: raw_info.vuln_long_msg.to_string(),
            span,
            help: help.map(|s| s.to_string()),
        }
    }
}

/*
This trait should be implemented by every enum of detectors (for each blockchain)
We cannot use this trait because it's not possible to make CONST functions in traits!
If in the future this is possible, we can use this trait to enforce the implementation of the functions
*/
pub trait DetectorImpl: std::fmt::Display {
    fn get_lint_message(&self) -> &'static str;

    #[cfg(feature = "lint_helper")]
    fn get_detector_info(&self) -> DetectorRawInfo;

    #[cfg(feature = "lint_helper")]
    fn span_lint_and_help<T: LintContext>(
        &self,
        cx: &T,
        lint: &'static Lint,
        span: Span,
        help: &str,
    );

    #[cfg(feature = "lint_helper")]
    fn span_lint<T: LintContext>(&self, cx: &T, lint: &'static Lint, span: Span);
}

impl DetectorImpl for SorobanDetector {
    fn get_lint_message(&self) -> &'static str {
        match self {
            SorobanDetector::AvoidCoreMemForget => SOROBAN_AVOID_CORE_MEM_FORGET_LINT_MESSAGE,
            SorobanDetector::InsufficientlyRandomValues => {
                SOROBAN_INSUFFICIENTLY_RANDOM_VALUES_LINT_MESSAGE
            }
            SorobanDetector::DivideBeforeMultiply => SOROBAN_DIVIDE_BEFORE_MULTIPLY_LINT_MESSAGE,
            SorobanDetector::OverflowCheck => SOROBAN_OVERFLOW_CHECK_LINT_MESSAGE,
            SorobanDetector::SetContractStorage => SOROBAN_SET_CONTRACT_STORAGE_LINT_MESSAGE,
            SorobanDetector::UnprotectedUpdateCurrentContractWasm => {
                SOROBAN_UNPROTECTED_UPDATE_CURRENT_CONTRACT_LINT_MESSAGE
            }
            SorobanDetector::UnsafeExpect => SOROBAN_UNSAFE_EXPECT_LINT_MESSAGE,
            SorobanDetector::UnsafeUnwrap => SOROBAN_UNSAFE_UNWRAP_LINT_MESSAGE,
            SorobanDetector::AvoidPanicError => SOROBAN_AVOID_PANIC_ERROR_LINT_MESSAGE,
            SorobanDetector::AvoidUnsafeBlock => SOROBAN_AVOID_UNSAFE_BLOCK_LINT_MESSAGE,
            SorobanDetector::DosUnboundedOperation => SOROBAN_DOS_UNBOUNDED_OPERATION_LINT_MESSAGE,
            SorobanDetector::SorobanVersion => SOROBAN_SOROBAN_VERSION_LINT_MESSAGE,
        }
    }
    #[cfg(feature = "lint_helper")]
    fn get_detector_info(&self) -> DetectorRawInfo {
        todo!()
    }

    #[cfg(feature = "lint_helper")]
    fn span_lint_and_help<T: LintContext>(
        &self,
        cx: &T,
        lint: &'static Lint,
        span: Span,
        help: &str,
    ) {
        send_to_server(span.clone(), Some(help), self.get_detector_info());
        //print_scout_output(*lint, span);
        span_lint_and_help_clippy(cx, lint, span, self.get_lint_message(), None, help);
    }

    #[cfg(feature = "lint_helper")]
    fn span_lint<T: LintContext>(&self, cx: &T, lint: &'static Lint, span: Span) {
        send_to_server(span.clone(), None, self.get_detector_info());
        //print_scout_output(*lint, span);
        span_lint_clippy(cx, lint, span, self.get_lint_message());
    }
}

impl DetectorImpl for InkDetector {
    /// Returns the lint message for the detector.
    fn get_lint_message(&self) -> &'static str {
        match self {
            InkDetector::AssertViolation => INK_ASSERT_VIOLATION_LINT_MESSAGE,
            InkDetector::AvoidCoreMemForget => INK_AVOID_CORE_MEM_FORGET_LINT_MESSAGE,
            InkDetector::AvoidFormatString => INK_AVOID_FORMAT_STRING_LINT_MESSAGE,
            InkDetector::DelegateCall => INK_DELEGATE_CALL_LINT_MESSAGE,
            InkDetector::DivideBeforeMultiply => INK_DIVIDE_BEFORE_MULTIPLY_LINT_MESSAGE,
            InkDetector::DosUnboundedOperation => INK_DOS_UNBOUNDED_OPERATION_LINT_MESSAGE,
            InkDetector::DosUnexpectedRevertWithVector => {
                INK_DOS_UNEXPECTED_REVERT_WITH_VECTOR_LINT_MESSAGE
            }
            InkDetector::InkVersion => INK_INK_VERSION_LINT_MESSAGE,
            InkDetector::InsufficientlyRandomValues => {
                INK_INSUFFICIENTLY_RANDOM_VALUES_LINT_MESSAGE
            }
            InkDetector::IntegerOverflowOrUnderflow => {
                INK_INTEGER_OVERFLOW_OR_UNDERFLOW_LINT_MESSAGE
            }
            InkDetector::IteratorsOverIndexing => INK_ITERATORS_OVER_INDEXING_LINT_MESSAGE,
            InkDetector::LazyDelegate => INK_LAZY_DELEGATE_LINT_MESSAGE,
            InkDetector::PanicError => INK_PANIC_ERROR_LINT_MESSAGE,
            InkDetector::Reentrancy1 => INK_REENTRANCY_LINT_MESSAGE,
            InkDetector::Reentrancy2 => INK_REENTRANCY_LINT_MESSAGE,
            InkDetector::SetCodeHash => INK_SET_CODE_HASH_LINT_MESSAGE,
            InkDetector::SetContractStorage => INK_SET_CONTRACT_STORAGE_LINT_MESSAGE,
            InkDetector::UnprotectedMappingOperation => {
                INK_UNPROTECTED_MAPPING_OPERATION_LINT_MESSAGE
            }
            InkDetector::UnprotectedSelfDestruct => INK_UNPROTECTED_SELF_DESTRUCT_LINT_MESSAGE,
            InkDetector::UnrestrictedTransferFrom => INK_UNRESTRICTED_TRANSFER_FROM_LINT_MESSAGE,
            InkDetector::UnsafeExpect => INK_UNSAFE_EXPECT_LINT_MESSAGE,
            InkDetector::UnsafeUnwrap => INK_UNSAFE_UNWRAP_LINT_MESSAGE,
            InkDetector::UnusedReturnEnum => INK_UNUSED_RETURN_ENUM_LINT_MESSAGE,
            InkDetector::ZeroOrTestAddress => INK_ZERO_OR_TEST_ADDRESS_LINT_MESSAGE,
        }
    }

    #[cfg(feature = "lint_helper")]
    fn get_detector_info(&self) -> DetectorRawInfo {
        match self {
            InkDetector::AssertViolation => INK_ASSERT_VIOLATION_LINT_INFO,
            InkDetector::AvoidCoreMemForget => INK_AVOID_CORE_MEM_FORGET_LINT_INFO,
            InkDetector::AvoidFormatString => INK_AVOID_FORMAT_STRING_LINT_INFO,
            InkDetector::DelegateCall => INK_DELEGATE_CALL_LINT_INFO,
            InkDetector::DivideBeforeMultiply => INK_DIVIDE_BEFORE_MULTIPLY_LINT_INFO,
            InkDetector::DosUnboundedOperation => INK_DOS_UNBOUNDED_OPERATION_LINT_INFO,
            InkDetector::DosUnexpectedRevertWithVector => {
                INK_DOS_UNEXPECTED_REVERT_WITH_VECTOR_LINT_INFO
            }
            InkDetector::InkVersion => INK_INK_VERSION_LINT_INFO,
            InkDetector::InsufficientlyRandomValues => INK_INSUFFICIENTLY_RANDOM_VALUES_LINT_INFO,
            InkDetector::IntegerOverflowOrUnderflow => INK_INTEGER_OVERFLOW_OR_UNDERFLOW_LINT_INFO,
            InkDetector::IteratorsOverIndexing => INK_ITERATORS_OVER_INDEXING_LINT_INFO,
            InkDetector::LazyDelegate => INK_LAZY_DELEGATE_LINT_INFO,
            InkDetector::PanicError => INK_PANIC_ERROR_LINT_INFO,
            InkDetector::Reentrancy1 => INK_REENTRANCY_LINT_INFO,
            InkDetector::Reentrancy2 => INK_REENTRANCY_LINT_INFO,
            InkDetector::SetCodeHash => INK_SET_CODE_HASH_LINT_INFO,
            InkDetector::SetContractStorage => INK_SET_CONTRACT_STORAGE_LINT_INFO,
            InkDetector::UnprotectedMappingOperation => INK_UNPROTECTED_MAPPING_OPERATION_LINT_INFO,
            InkDetector::UnprotectedSelfDestruct => INK_UNPROTECTED_SELF_DESTRUCT_LINT_INFO,
            InkDetector::UnrestrictedTransferFrom => INK_UNRESTRICTED_TRANSFER_FROM_LINT_INFO,
            InkDetector::UnsafeExpect => INK_UNSAFE_EXPECT_LINT_INFO,
            InkDetector::UnsafeUnwrap => INK_UNSAFE_UNWRAP_LINT_INFO,
            InkDetector::UnusedReturnEnum => INK_UNUSED_RETURN_ENUM_LINT_INFO,
            InkDetector::ZeroOrTestAddress => INK_ZERO_OR_TEST_ADDRESS_LINT_INFO,
        }
    }

    #[cfg(feature = "lint_helper")]
    fn span_lint_and_help<T: LintContext>(
        &self,
        cx: &T,
        lint: &'static Lint,
        span: Span,
        help: &str,
    ) {
        send_to_server(span.clone(), Some(help), self.get_detector_info());
        //print_scout_output(*lint, span);
        span_lint_and_help_clippy(cx, lint, span, self.get_lint_message(), None, help);
    }

    #[cfg(feature = "lint_helper")]
    fn span_lint<T: LintContext>(&self, cx: &T, lint: &'static Lint, span: Span) {
        send_to_server(span.clone(), None, self.get_detector_info());
        //print_scout_output(*lint, span);
        span_lint_clippy(cx, lint, span, self.get_lint_message());
    }
}

#[cfg(feature = "lint_helper")]
#[tokio::main]
async fn send_to_server(
    span: Span,
    help: Option<&str>,
    detector: DetectorRawInfo,
) -> anyhow::Result<()> {
    use std::fmt::format;

    use crate::DetectorSocketClient;

    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 1177);

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);
    let client =
        DetectorSocketClient::new(client::Config::default(), transport.await.unwrap()).spawn();
    client
        .push_finding(
            context::current(),
            DetectorInfo::from_raw(detector, format!("{:?}", span), help),
        )
        .await
        .unwrap();

    Ok(())
}
/* 

#[cfg(feature = "lint_helper")]
fn print_scout_output(lint: Lint, span: Span) {
    let span_debug_string: Vec<String> = format!("{:?}", span)
        .split(':')
        .map(|s| s.trim().to_string())
        .collect();

    let no_span_detectors = ["OVERFLOW_CHECK", "INK_VERSION", "SOROBAN_VERSION"];

    if no_span_detectors.contains(&lint.name.to_owned().as_str()) {
        let span = json!({
            "physicalLocation": {
                "artifactLocation": {
                    "uri": "Cargo.toml",
                },
                "region": {
                    "startLine": 1,
                    "startColumn": 1,
                    "endLine": 1,
                    "endColumn": 1,
                }
            }
        });

        println!("scout-internal:{}@{}", lint.name, span);
        return;
    }

    let span = json!({
        "physicalLocation": {
            "artifactLocation": {
                "uri": span_debug_string[0],
            },
            "region": {
                "startLine": 0,
                "startColumn": 0,
                "endLine": 0,
                "endColumn": 0,            }
            }
    });
    println!("scout-internal:{}@{}", lint.name, span);
}

*/