#[cfg(feature = "lint_helper")]
extern crate rustc_driver;
#[cfg(feature = "lint_helper")]
extern crate rustc_errors;
#[cfg(feature = "lint_helper")]
extern crate rustc_lint;
#[cfg(feature = "lint_helper")]
extern crate rustc_span;

mod lint_message;

#[cfg(feature = "lint_helper")]
use clippy_utils::diagnostics::{
    span_lint as span_lint_clippy, span_lint_and_help as span_lint_and_help_clippy,
};
use lint_message::*;
#[cfg(feature = "lint_helper")]
use rustc_lint::{Lint, LintContext};
#[cfg(feature = "lint_helper")]
use rustc_span::Span;
#[cfg(feature = "lint_helper")]
use serde_json::json;
use strum::{Display, EnumIter};

/// Available detectors.
#[derive(Debug, Display, Clone, EnumIter, PartialEq, Eq, Hash)]
#[strum(serialize_all = "kebab-case")]
pub enum Detector {
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

impl Detector {
    /// Returns the lint message for the detector.
    pub const fn get_lint_message(&self) -> &'static str {
        match self {
            Detector::AssertViolation => ASSERT_VIOLATION_LINT_MESSAGE,
            Detector::AvoidCoreMemForget => AVOID_CORE_MEM_FORGET_LINT_MESSAGE,
            Detector::AvoidFormatString => AVOID_FORMAT_STRING_LINT_MESSAGE,
            Detector::DelegateCall => DELEGATE_CALL_LINT_MESSAGE,
            Detector::DivideBeforeMultiply => DIVIDE_BEFORE_MULTIPLY_LINT_MESSAGE,
            Detector::DosUnboundedOperation => DOS_UNBOUNDED_OPERATION_LINT_MESSAGE,
            Detector::DosUnexpectedRevertWithVector => {
                DOS_UNEXPECTED_REVERT_WITH_VECTOR_LINT_MESSAGE
            }
            Detector::InkVersion => INK_VERSION_LINT_MESSAGE,
            Detector::InsufficientlyRandomValues => INSUFFICIENTLY_RANDOM_VALUES_LINT_MESSAGE,
            Detector::IntegerOverflowOrUnderflow => INTEGER_OVERFLOW_OR_UNDERFLOW_LINT_MESSAGE,
            Detector::IteratorsOverIndexing => ITERATORS_OVER_INDEXING_LINT_MESSAGE,
            Detector::LazyDelegate => LAZY_DELEGATE_LINT_MESSAGE,
            Detector::PanicError => PANIC_ERROR_LINT_MESSAGE,
            Detector::Reentrancy1 => REENTRANCY_LINT_MESSAGE,
            Detector::Reentrancy2 => REENTRANCY_LINT_MESSAGE,
            Detector::SetCodeHash => SET_CODE_HASH_LINT_MESSAGE,
            Detector::SetContractStorage => SET_CONTRACT_STORAGE_LINT_MESSAGE,
            Detector::UnprotectedMappingOperation => UNPROTECTED_MAPPING_OPERATION_LINT_MESSAGE,
            Detector::UnprotectedSelfDestruct => UNPROTECTED_SELF_DESTRUCT_LINT_MESSAGE,
            Detector::UnrestrictedTransferFrom => UNRESTRICTED_TRANSFER_FROM_LINT_MESSAGE,
            Detector::UnsafeExpect => UNSAFE_EXPECT_LINT_MESSAGE,
            Detector::UnsafeUnwrap => UNSAFE_UNWRAP_LINT_MESSAGE,
            Detector::UnusedReturnEnum => UNUSED_RETURN_ENUM_LINT_MESSAGE,
            Detector::ZeroOrTestAddress => ZERO_OR_TEST_ADDRESS_LINT_MESSAGE,
        }
    }

    #[cfg(feature = "lint_helper")]
    pub fn span_lint_and_help<T: LintContext>(
        &self,
        cx: &T,
        lint: &'static Lint,
        span: Span,
        help: &str,
    ) {
        print_scout_output(*lint, span);
        span_lint_and_help_clippy(cx, lint, span, self.get_lint_message(), None, help);
    }

    #[cfg(feature = "lint_helper")]
    pub fn span_lint<T: LintContext>(&self, cx: &T, lint: &'static Lint, span: Span) {
        print_scout_output(*lint, span);
        span_lint_clippy(cx, lint, span, self.get_lint_message());
    }
}

#[cfg(feature = "lint_helper")]
fn print_scout_output(lint: Lint, span: Span) {
    let span_debug_string: Vec<String> = format!("{:?}", span)
        .split(':')
        .map(|s| s.trim().to_string())
        .collect();

    let mut span_info: [u32; 4] = [0, 0, 0, 0];
    if span_debug_string.len() == 5 {
        for i in 0..3 {
            span_info[i] = span_debug_string[i + 1].parse::<u32>().unwrap();
        }
        span_info[3] = span_debug_string[4].split(' ').collect::<Vec<&str>>()[0]
            .trim()
            .parse::<u32>()
            .unwrap();
    }

    let span = json!({
        "physicalLocation": {
            "artifactLocation": {
                "uri": span_debug_string[0],
            },
            "region": {
                "startLine": span_info[0],
                "startColumn": span_info[1],
                "endLine": span_info[2],
                "endColumn": span_info[3],
            }
        }
    });
    println!("scout-internal:{}@{}", lint.name, span);
}
