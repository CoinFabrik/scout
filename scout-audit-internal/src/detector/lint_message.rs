pub const ASSERT_VIOLATION_LINT_MESSAGE: &str =
    "Assert causes panic. Instead, return a proper error.";
pub const AVOID_CORE_MEM_FORGET_LINT_MESSAGE: &str =
    "Using `core::mem::forget` is not recommended.";
pub const AVOID_FORMAT_STRING_LINT_MESSAGE: &str = "The format! macro should not be used.";
pub const DELEGATE_CALL_LINT_MESSAGE: &str = "Passing arguments to the target of a delegate call is not safe, as it allows the caller to set a malicious hash as the target.";
pub const DIVIDE_BEFORE_MULTIPLY_LINT_MESSAGE: &str =
    "Division before multiplication might result in a loss of precision";
pub const DOS_UNBOUNDED_OPERATION_LINT_MESSAGE: &str =
"In order to prevent a single transaction from consuming all the gas in a block, unbounded operations must be avoided";
pub const DOS_UNEXPECTED_REVERT_WITH_VECTOR_LINT_MESSAGE: &str =
    "This vector operation is called without access control";
pub const INK_VERSION_LINT_MESSAGE: &str = "Use the latest version of ink!";
pub const INSUFFICIENTLY_RANDOM_VALUES_LINT_MESSAGE: &str = "In order to prevent randomness manipulations by validators block_timestamp should not be used as random number source";
pub const INTEGER_OVERFLOW_OR_UNDERFLOW_LINT_MESSAGE: &str = "Potential for integer arithmetic overflow/underflow. Consider checked, wrapping or saturating arithmetic.";
pub const ITERATORS_OVER_INDEXING_LINT_MESSAGE: &str =
    "Hardcoding an index could lead to panic if the top bound is out of bounds.";
pub const LAZY_DELEGATE_LINT_MESSAGE: &str = "Delegate call with non-lazy, non-mapping storage";
pub const PANIC_ERROR_LINT_MESSAGE: &str = "The panic! macro is used to stop execution when a condition is not met. This is useful for testing and prototyping, but should be avoided in production code";
pub const REENTRANCY_LINT_MESSAGE:&str = "External calls could open the opportunity for a malicious contract to execute any arbitrary code";
pub const SET_CODE_HASH_LINT_MESSAGE: &str = "This set_code_hash is called without access control";
pub const SET_CONTRACT_STORAGE_LINT_MESSAGE:&str = "Abitrary users should not have control over keys because it implies writing any value of left mapping, lazy variable, or the main struct of the contract located in position 0 of the storage";
pub const UNPROTECTED_MAPPING_OPERATION_LINT_MESSAGE: &str = "This mapping operation is called without access control on a different key than the caller's address";
pub const UNPROTECTED_SELF_DESTRUCT_LINT_MESSAGE: &str =
    "This terminate_contract is called without access control";
pub const UNRESTRICTED_TRANSFER_FROM_LINT_MESSAGE: &str =
    "This argument comes from a user-supplied argument";
pub const UNSAFE_EXPECT_LINT_MESSAGE: &str = "Unsafe usage of `expect`";
pub const UNSAFE_UNWRAP_LINT_MESSAGE: &str = "Unsafe usage of `unwrap`";
pub const UNUSED_RETURN_ENUM_LINT_MESSAGE: &str = "Unused return enum";
pub const ZERO_OR_TEST_ADDRESS_LINT_MESSAGE: &str =
    "Not checking for a zero-address could lead to a locked contract";
