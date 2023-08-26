use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub enum QueryError {
    /// A database entry for the specified record already exists
    RecordExists,

    /// A database index entry for the specified record already exists
    IndexExists,

    /// This error is used for ignoring a document when processing a query
    Ignore,

    /// This error is used for breaking a loop in a foreach statement
    Break,

    /// This error is used for skipping a loop in a foreach statement
    Continue,

    /// The database encountered unreachable logic
    Unreachable,

    /// A custom error has been thrown
    Thrown,

    /// There was a problem with the underlying datastore
    Ds,

    /// There was a problem with a datastore transaction
    Tx,

    /// There was an error when starting a new datastore transaction
    TxFailure,

    /// The transaction was already cancelled or committed
    TxFinished,

    /// The current transaction was created as read-only
    TxReadonly,

    /// The conditional value in the request was not equal
    TxConditionNotMet,

    /// The key being inserted in the transaction already exists
    TxKeyAlreadyExists,

    /// The key exceeds a limit set by the KV store
    TxKeyTooLarge,

    /// The value exceeds a limit set by the KV store
    TxValueTooLarge,

    /// The transaction writes too much data for the KV store
    TxTooLarge,

    /// No namespace has been selected
    NsEmpty,

    /// No database has been selected
    DbEmpty,

    /// No SQL query has been specified
    QueryEmpty,

    /// There was an error with the SQL query
    QueryRemaining,

    /// There was an error with authentication
    InvalidAuth,

    /// Auth was expected to be set but was unknown
    UnknownAuth,

    /// There was an error with the SQL query
    InvalidQuery,

    /// There was an error with the provided JSON Patch
    InvalidPatch,

    /// Given test operation failed for JSON Patch
    PatchTest,

    /// Remote HTTP request functions are not enabled
    HttpDisabled,

    /// it is not possible to set a variable with the specified name
    InvalidParam,

    InvalidField,

    InvalidSplit,

    InvalidOrder,

    InvalidGroup,

    /// The LIMIT clause must evaluate to a positive integer
    InvalidLimit,

    /// The START clause must evaluate to a positive integer
    InvalidStart,

    /// There was an error with the provided JavaScript code
    InvalidScript,

    /// There was a problem running the specified function
    InvalidFunction,

    /// The wrong quantity or magnitude of arguments was given for the specified function
    InvalidArguments,

    /// The URL is invalid
    InvalidUrl,

    /// The query timedout
    QueryTimedout,

    /// The query did not execute, because the transaction was cancelled
    QueryCancelled,

    /// The query did not execute, because the transaction has failed
    QueryNotExecuted,

    /// The query did not execute, because the transaction has failed (with a message)
    QueryNotExecutedDetail,

    /// The permissions do not allow for changing to the specified namespace
    NsNotAllowed,

    /// The permissions do not allow for changing to the specified database
    DbNotAllowed,

    /// The requested namespace does not exist
    NsNotFound,

    /// The requested namespace token does not exist
    NtNotFound,

    /// The requested namespace login does not exist
    NlNotFound,

    /// The requested database does not exist
    DbNotFound,

    /// The requested database token does not exist
    DtNotFound,

    /// The requested database login does not exist
    DlNotFound,

    /// The requested function does not exist
    FcNotFound,

    /// The requested scope does not exist
    ScNotFound,

    // The cluster node already exists
    ClAlreadyExists,

    // The cluster node does not exist
    NdNotFound,

    /// The requested scope token does not exist
    StNotFound,

    /// The requested param does not exist
    PaNotFound,

    /// The requested table does not exist
    TbNotFound,

    /// The requested live query does not exist
    LvNotFound,

    /// The requested cluster live query does not exist
    LqNotFound,

    /// The requested analyzer does not exist
    AzNotFound,

    /// The requested analyzer does not exist
    IxNotFound,

    /// The requested root user does not exist
    UserRootNotFound,

    /// The requested namespace user does not exist
    UserNsNotFound,

    /// The requested database user does not exist
    UserDbNotFound,

    /// Unable to perform the realtime query
    RealtimeDisabled,

    /// Reached excessive computation depth due to functions, subqueries, or futures
    ComputationDepthExceeded,

    /// Can not execute statement using the specified value
    InvalidStatementTarget,

    /// Can not execute CREATE statement using the specified value
    CreateStatement,

    /// Can not execute UPDATE statement using the specified value
    UpdateStatement,

    /// Can not execute RELATE statement using the specified value
    RelateStatement,

    /// Can not execute DELETE statement using the specified value
    DeleteStatement,

    /// Can not execute INSERT statement using the specified value
    InsertStatement,

    /// Can not execute LIVE statement using the specified value
    LiveStatement,

    /// Can not execute KILL statement using the specified id
    KillStatement,

    /// The permissions do not allow this query to be run on this table
    TablePermissions,

    /// The specified table can not be written as it is setup as a foreign table view
    TableIsView,

    /// The specified field did not conform to the field type check
    FieldCheck,

    /// The specified field did not conform to the field ASSERT clause
    FieldValue,

    /// Found a record id for the record but we are creating a specific record
    IdMismatch,

    /// Found a record id for the record but this is not a valid id
    IdInvalid,

    /// Unable to coerce to a value to another value
    CoerceTo,

    /// Unable to convert a value to another value
    ConvertTo,

    /// Unable to coerce to a value to another value
    LengthInvalid,

    /// Cannot perform addition
    TryAdd,

    /// Cannot perform subtraction
    TrySub,

    /// Cannot perform multiplication
    TryMul,

    /// Cannot perform division
    TryDiv,

    /// Cannot perform power
    TryPow,

    /// Cannot perform negation
    TryNeg,

    /// It's is not possible to convert between the two types
    TryFrom,

    /// There was an error processing a remote HTTP request
    Http,

    /// There was an error processing a value in parallel
    Channel,

    /// Represents an underlying error with IO encoding / decoding
    Io,

    /// Represents an error when encoding a key-value entry
    Encode,

    /// Represents an error when decoding a key-value entry
    Decode,

    /// Represents an underlying error with versioned data encoding / decoding
    Revision,

    /// The index has been found to be inconsistent
    CorruptedIndex,

    /// The query planner did not find an index able to support the match @@ operator on a given expression
    NoIndexFoundForMatch,

    /// Represents an error when analyzing a value
    AnalyzerError,

    /// Represents an error when trying to highlight a value
    HighlightError,

    /// Represents an underlying error with Bincode serializing / deserializing
    Bincode,

    /// Represents an underlying error with FST
    FstError,

    /// Represents an underlying error while reading UTF8 characters
    Utf8Error,

    /// The feature has not yet being implemented
    FeatureNotYetImplemented,

    /// Duplicated match references are not allowed
    DuplicatedMatchRef,

    /// Represents a failure in timestamp arithmetic related to database internals
    TimestampOverflow,

    /// Internal server error
    /// This should be used extremely sporadically, since we lose the type of error as a consequence
    /// There will be times when it is useful, such as with unusual type conversion errors
    Internal,

    /// Unimplemented functionality
    Unimplemented,

    CorruptedVersionstampInKey,

    /// Invalid level
    InvalidLevel,

    /// Represents an underlying IAM error
    IamError,

    //
    // Capabilities
    //
    /// Scripting is not allowed
    ScriptingNotAllowed,

    /// Function is not allowed
    FunctionNotAllowed,

    /// Network target is not allowed
    NetTargetNotAllowed,

    /// Statement has been deprecated
    #[default]
    Deprecated,
}

static REGEX_LIST: Lazy<[Regex; 115]> = Lazy::new(|| {
    [
        Regex::new(r"\ADatabase record `.*` already exists\z").unwrap(), // RecordExists
        Regex::new(r"\ADatabase index `.*` already contains .*, with record `.*`\z").unwrap(), // IndexExists
        Regex::new(r"\AConditional clause is not truthy\z").unwrap(), // Ignore
        Regex::new(r"\ABreak statement has been reached\z").unwrap(), // Break
        Regex::new(r"\AContinue statement has been reached\z").unwrap(), // Continue
        Regex::new(r"\AThe database encountered unreachable logic\z").unwrap(), // Unreachable
        Regex::new(r"\AAn error occurred: .*\z").unwrap(), // Thrown
        Regex::new(r"\AThere was a problem with the underlying datastore: .*\z").unwrap(), // Ds
        Regex::new(r"\AThere was a problem with a datastore transaction: .*\z").unwrap(), // Tx
        Regex::new(r"\AThere was an error when starting a new datastore transaction\z").unwrap(), // TxFailure
        Regex::new(r"\ACouldn't update a finished transaction\z").unwrap(), // TxFinished
        Regex::new(r"\ACouldn't write to a read only transaction\z").unwrap(), // TxReadonly
        Regex::new(r"\AValue being checked was not correct\z").unwrap(), // TxConditionNotMet
        Regex::new(r"\AThe key being inserted already exists\z").unwrap(), // TxKeyAlreadyExists
        Regex::new(r"\ARecord id or key is too large\z").unwrap(), // TxKeyTooLarge
        Regex::new(r"\ARecord or value is too large\z").unwrap(), // TxValueTooLarge
        Regex::new(r"\ATransaction is too large\z").unwrap(), // TxTooLarge
        Regex::new(r"\ASpecify a namespace to use\z").unwrap(), // NsEmpty
        Regex::new(r"\ASpecify a database to use\z").unwrap(), // DbEmpty
        Regex::new(r"\ASpecify some SQL code to execute\z").unwrap(), // QueryEmpty
        Regex::new(r"\AThe SQL query was not parsed fully\z").unwrap(), // QueryRemaining
        Regex::new(r"\AThere was a problem with authentication\z").unwrap(), // InvalidAuth
        Regex::new(r"\AAuth was expected to be set but was unknown\z").unwrap(), // UnknownAuth
        Regex::new(r"\AParse error on line .* at character .* when parsing '.*'\z").unwrap(), // InvalidQuery
        Regex::new(r"\AThe JSON Patch contains invalid operations. .*\z").unwrap(), // InvalidPatch
        Regex::new(r"\AGiven test operation failed for JSON Patch. Expected `.*`, but got `.*` instead.\z").unwrap(), // PatchTest
        Regex::new(r"\ARemote HTTP request functions are not enabled\z").unwrap(), // HttpDisabled
        Regex::new(r"\AFound '.*' but it is not possible to set a variable with this name\z").unwrap(), // InvalidParam
        Regex::new(r"\AFound '.*' in SELECT clause on line .*, but field is not an aggregate function, and is not present in GROUP BY expression\z").unwrap(), // InvalidField
        Regex::new(r"\AFound '.*' in SPLIT ON clause on line .*, but field is not present in SELECT expression\z").unwrap(), // InvalidSplit
        Regex::new(r"\AFound '.*' in ORDER BY clause on line .*, but field is not present in SELECT expression\z").unwrap(), // InvalidOrder
        Regex::new(r"\AFound '.*' in GROUP BY clause on line .*, but field is not present in SELECT expression\z").unwrap(), // InvalidGroup
        Regex::new(r"\AFound .* but the LIMIT clause must evaluate to a positive integer\z").unwrap(), // InvalidLimit
        Regex::new(r"\AFound .* but the START clause must evaluate to a positive integer\z").unwrap(), // InvalidStart
        Regex::new(r"\AProblem with embedded script function. .*\z").unwrap(), // InvalidScript
        Regex::new(r"\AThere was a problem running the .*() function. .*\z").unwrap(), // InvalidFunction
        Regex::new(r"\AIncorrect arguments for function .*(). .*\z").unwrap(), // InvalidArguments
        Regex::new(r"\AThe URL `.*` is invalid\z").unwrap(), // InvalidUrl
        Regex::new(r"\AThe query was not executed because it exceeded the timeout\z").unwrap(), // QueryTimedout
        Regex::new(r"\AThe query was not executed due to a cancelled transaction\z").unwrap(), // QueryCancelled
        Regex::new(r"\AThe query was not executed due to a failed transaction\z").unwrap(), // QueryNotExecuted
        Regex::new(r"\AThe query was not executed due to a failed transaction. .*\z").unwrap(), // QueryNotExecutedDetail
        Regex::new(r"\AYou don't have permission to change to the .* namespace\z").unwrap(), // NsNotAllowed
        Regex::new(r"\AYou don't have permission to change to the .* database\z").unwrap(), // DbNotAllowed
        Regex::new(r"\AThe namespace '.*' does not exist\z").unwrap(), // NsNotFound
        Regex::new(r"\AThe namespace token '.*' does not exist\z").unwrap(), // NtNotFound
        Regex::new(r"\AThe namespace login '.*' does not exist\z").unwrap(), // NlNotFound
        Regex::new(r"\AThe database '.*' does not exist\z").unwrap(), // DbNotFound
        Regex::new(r"\AThe database token '.*' does not exist\z").unwrap(), // DtNotFound
        Regex::new(r"\AThe database login '.*' does not exist\z").unwrap(), // DlNotFound
        Regex::new(r"\AThe function 'fn::.*' does not exist\z").unwrap(), // FcNotFound
        Regex::new(r"\AThe scope '.*' does not exist\z").unwrap(), // ScNotFound
        Regex::new(r"\AThe node '.*' already exists\z").unwrap(), // ClAlreadyExists
        Regex::new(r"\AThe node '.*' does not exist\z").unwrap(), // NdNotFound
        Regex::new(r"\AThe scope token '.*' does not exist\z").unwrap(), // StNotFound
        Regex::new(r"\AThe param '\$.*' does not exist\z").unwrap(), // PaNotFound
        Regex::new(r"\AThe table '.*' does not exist\z").unwrap(), // TbNotFound
        Regex::new(r"\AThe live query '.*' does not exist\z").unwrap(), // LvNotFound
        Regex::new(r"\AThe cluster live query '.*' does not exist\z").unwrap(), // LqNotFound
        Regex::new(r"\AThe analyzer '.*' does not exist\z").unwrap(), // AzNotFound
        Regex::new(r"\AThe index '.*' does not exist\z").unwrap(), // IxNotFound
        Regex::new(r"\AThe root user '.*' does not exist\z").unwrap(), // UserRootNotFound
        Regex::new(r"\AThe user '.*' does not exist in the namespace '.*'\z").unwrap(), // UserNsNotFound
        Regex::new(r"\AThe user '.*' does not exist in the database '.*'\z").unwrap(), // UserDbNotFound
        Regex::new(r"\AUnable to perform the realtime query\z").unwrap(), // RealtimeDisabled
        Regex::new(r"\AReached excessive computation depth due to functions, subqueries, or futures\z").unwrap(), // ComputationDepthExceeded
        Regex::new(r"\ACan not execute statement using value '.*'\z").unwrap(), // InvalidStatementTarget
        Regex::new(r"\ACan not execute CREATE statement using value '.*'\z").unwrap(), // CreateStatement
        Regex::new(r"\ACan not execute UPDATE statement using value '.*'\z").unwrap(), // UpdateStatement
        Regex::new(r"\ACan not execute RELATE statement using value '.*'\z").unwrap(), // RelateStatement
        Regex::new(r"\ACan not execute DELETE statement using value '.*'\z").unwrap(), // DeleteStatement
        Regex::new(r"\ACan not execute INSERT statement using value '.*'\z").unwrap(), // InsertStatement
        Regex::new(r"\ACan not execute LIVE statement using value '.*'\z").unwrap(), // LiveStatement
        Regex::new(r"\ACan not execute KILL statement using id '.*'\z").unwrap(), // KillStatement
        Regex::new(r"\AYou don't have permission to run this query on the `.*` table\z").unwrap(), // TablePermissions
        Regex::new(r"\AUnable to write to the `.*` table while setup as a view\z").unwrap(), // TableIsView
        Regex::new(r"\AFound .* for field `.*`, with record `.*`, but expected a .*\z").unwrap(), // FieldCheck
        Regex::new(r"\AFound .* for field `.*`, with record `.*`, but field must conform to: .*\z").unwrap(), // FieldValue
        Regex::new(r"\AFound .* for the id field, but a specific record has been specified\z").unwrap(), // IdMismatch
        Regex::new(r"\AFound .* for the Record ID but this is not a valid id\z").unwrap(), // IdInvalid
        Regex::new(r"\AExpected a .* but found .*\z").unwrap(), // CoerceTo
        Regex::new(r"\AExpected a .* but cannot convert .* into a .*\z").unwrap(), // ConvertTo
        Regex::new(r"\AExpected a .* but the array had .* items\z").unwrap(), // LengthInvalid
        Regex::new(r"\ACannot perform addition with '.*' and '.*'\z").unwrap(), // TryAdd
        Regex::new(r"\ACannot perform subtraction with '.*' and '.*'\z").unwrap(), // TrySub
        Regex::new(r"\ACannot perform multiplication with '.*' and '.*'\z").unwrap(), // TryMul
        Regex::new(r"\ACannot perform division with '.*' and '.*'\z").unwrap(), // TryDiv
        Regex::new(r"\ACannot raise the value '.*' with '.*'\z").unwrap(), // TryPow
        Regex::new(r"\ACannot negate the value '.*'\z").unwrap(), // TryNeg
        Regex::new(r"\ACannot convert from '.*' to '.*'\z").unwrap(), // TryFrom
        Regex::new(r"\AThere was an error processing a remote HTTP request: .*\z").unwrap(), // Http
        Regex::new(r"\AThere was an error processing a value in parallel: .*\z").unwrap(), // Channel
        Regex::new(r"\AI/O error: .*\z").unwrap(), // Io
        Regex::new(r"\AKey encoding error: .*\z").unwrap(), // Encode
        Regex::new(r"\AKey decoding error: .*\z").unwrap(), // Decode
        Regex::new(r"\AVersioned error: .*\z").unwrap(), // Revision
        Regex::new(r"\AIndex is corrupted\z").unwrap(), // CorruptedIndex
        Regex::new(r"\AThere was no suitable full-text index supporting the expression '.*'\z").unwrap(), // NoIndexFoundForMatch
        Regex::new(r"\AA value can't be analyzed: .*\z").unwrap(), // AnalyzerError
        Regex::new(r"\AA value can't be highlighted: .*\z").unwrap(), // HighlightError
        Regex::new(r"\ABincode error: .*\z").unwrap(), // Bincode
        Regex::new(r"\AFstError error: .*\z").unwrap(), // FstError
        Regex::new(r"\AUtf8 error: .*\z").unwrap(), // Utf8Error
        Regex::new(r"\AFeature not yet implemented: .*\z").unwrap(), // FeatureNotYetImplemented
        Regex::new(r"\ADuplicated Match reference: .*\z").unwrap(), // DuplicatedMatchRef
        Regex::new(r"\ATimestamp arithmetic error: .*\z").unwrap(), // TimestampOverflow
        Regex::new(r"\AInternal database error: .*\z").unwrap(), // Internal
        Regex::new(r"\AUnimplemented functionality: .*\z").unwrap(), // Unimplemented
        Regex::new(r"\AVersionstamp in key is corrupted: .*\z").unwrap(), // CorruptedVersionstampInKey
        Regex::new(r"\AInvalid level '.*'\z").unwrap(), // InvalidLevel
        Regex::new(r"\AIAM error: .*\z").unwrap(), // IamError
        Regex::new(r"\AScripting functions are not allowed\z").unwrap(), // ScriptingNotAllowed
        Regex::new(r"\AFunction '.*' is not allowed to be executed\z").unwrap(), // FunctionNotAllowed
        Regex::new(r"\AAccess to network target '.*' is not allowed\z").unwrap(), // NetTargetNotAllowed
        Regex::new(r"\A.*\z").unwrap(), // Deprecated
    ]
});

impl QueryError {
    /// Create a regex from an error type
    pub fn from_error_string(s: &str) -> QueryError {
        for (i, re) in REGEX_LIST.iter().enumerate() {
            if re.is_match(s) {
                return match i {
                    0 => QueryError::RecordExists,
                    1 => QueryError::IndexExists,
                    2 => QueryError::Ignore,
                    3 => QueryError::Break,
                    4 => QueryError::Continue,
                    5 => QueryError::Unreachable,
                    6 => QueryError::Thrown,
                    7 => QueryError::Ds,
                    8 => QueryError::Tx,
                    9 => QueryError::TxFailure,
                    10 => QueryError::TxFinished,
                    11 => QueryError::TxReadonly,
                    12 => QueryError::TxConditionNotMet,
                    13 => QueryError::TxKeyAlreadyExists,
                    14 => QueryError::TxKeyTooLarge,
                    15 => QueryError::TxValueTooLarge,
                    16 => QueryError::TxTooLarge,
                    17 => QueryError::NsEmpty,
                    18 => QueryError::DbEmpty,
                    19 => QueryError::QueryEmpty,
                    20 => QueryError::QueryRemaining,
                    21 => QueryError::InvalidAuth,
                    22 => QueryError::UnknownAuth,
                    23 => QueryError::InvalidQuery,
                    24 => QueryError::InvalidPatch,
                    25 => QueryError::PatchTest,
                    26 => QueryError::HttpDisabled,
                    27 => QueryError::InvalidParam,
                    28 => QueryError::InvalidField,
                    29 => QueryError::InvalidSplit,
                    30 => QueryError::InvalidOrder,
                    31 => QueryError::InvalidGroup,
                    32 => QueryError::InvalidLimit,
                    33 => QueryError::InvalidStart,
                    34 => QueryError::InvalidScript,
                    35 => QueryError::InvalidFunction,
                    36 => QueryError::InvalidArguments,
                    37 => QueryError::InvalidUrl,
                    38 => QueryError::QueryTimedout,
                    39 => QueryError::QueryCancelled,
                    40 => QueryError::QueryNotExecuted,
                    41 => QueryError::QueryNotExecutedDetail,
                    42 => QueryError::NsNotAllowed,
                    43 => QueryError::DbNotAllowed,
                    44 => QueryError::NsNotFound,
                    45 => QueryError::NtNotFound,
                    46 => QueryError::NlNotFound,
                    47 => QueryError::DbNotFound,
                    48 => QueryError::DtNotFound,
                    49 => QueryError::DlNotFound,
                    50 => QueryError::FcNotFound,
                    51 => QueryError::ScNotFound,
                    52 => QueryError::ClAlreadyExists,
                    53 => QueryError::NdNotFound,
                    54 => QueryError::StNotFound,
                    55 => QueryError::PaNotFound,
                    56 => QueryError::TbNotFound,
                    57 => QueryError::LvNotFound,
                    58 => QueryError::LqNotFound,
                    59 => QueryError::AzNotFound,
                    60 => QueryError::IxNotFound,
                    61 => QueryError::UserRootNotFound,
                    62 => QueryError::UserNsNotFound,
                    63 => QueryError::UserDbNotFound,
                    64 => QueryError::RealtimeDisabled,
                    65 => QueryError::ComputationDepthExceeded,
                    66 => QueryError::InvalidStatementTarget,
                    67 => QueryError::CreateStatement,
                    68 => QueryError::UpdateStatement,
                    69 => QueryError::RelateStatement,
                    70 => QueryError::DeleteStatement,
                    71 => QueryError::InsertStatement,
                    72 => QueryError::LiveStatement,
                    73 => QueryError::KillStatement,
                    74 => QueryError::TablePermissions,
                    75 => QueryError::TableIsView,
                    76 => QueryError::FieldCheck,
                    77 => QueryError::FieldValue,
                    78 => QueryError::IdMismatch,
                    79 => QueryError::IdInvalid,
                    80 => QueryError::CoerceTo,
                    81 => QueryError::ConvertTo,
                    82 => QueryError::LengthInvalid,
                    83 => QueryError::TryAdd,
                    84 => QueryError::TrySub,
                    85 => QueryError::TryMul,
                    86 => QueryError::TryDiv,
                    87 => QueryError::TryPow,
                    88 => QueryError::TryNeg,
                    89 => QueryError::TryFrom,
                    90 => QueryError::Http,
                    91 => QueryError::Channel,
                    92 => QueryError::Io,
                    93 => QueryError::Encode,
                    94 => QueryError::Decode,
                    95 => QueryError::Revision,
                    96 => QueryError::CorruptedIndex,
                    97 => QueryError::NoIndexFoundForMatch,
                    98 => QueryError::AnalyzerError,
                    99 => QueryError::HighlightError,
                    100 => QueryError::Bincode,
                    101 => QueryError::FstError,
                    102 => QueryError::Utf8Error,
                    103 => QueryError::FeatureNotYetImplemented,
                    104 => QueryError::DuplicatedMatchRef,
                    105 => QueryError::TimestampOverflow,
                    106 => QueryError::Internal,
                    107 => QueryError::Unimplemented,
                    108 => QueryError::CorruptedVersionstampInKey,
                    109 => QueryError::InvalidLevel,
                    110 => QueryError::IamError,
                    111 => QueryError::ScriptingNotAllowed,
                    112 => QueryError::FunctionNotAllowed,
                    113 => QueryError::NetTargetNotAllowed,
                    114 => QueryError::Deprecated,
                    _ => QueryError::default(),
                };
            }
        }

        QueryError::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore() {
        let error_string = "Conditional clause is not truthy";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Ignore);
    }

    #[test]
    fn r#break() {
        let error_string = "Break statement has been reached";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Break);
    }

    #[test]
    fn r#continue() {
        let error_string = "Continue statement has been reached";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Continue);
    }

    #[test]
    fn unreachable() {
        let error_string = "The database encountered unreachable logic";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Unreachable);
    }

    #[test]
    fn thrown() {
        let error_string = "An error occurred: error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Thrown);
    }

    #[test]
    fn ds() {
        let error_string = "There was a problem with the underlying datastore: error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Ds);
    }

    #[test]
    fn tx() {
        let error_string = "There was a problem with a datastore transaction: error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Tx);
    }

    #[test]
    fn tx_failure() {
        let error_string = "There was an error when starting a new datastore transaction";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxFailure);
    }

    #[test]
    fn tx_finished() {
        let error_string = "Couldn't update a finished transaction";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxFinished);
    }

    #[test]
    fn tx_readonly() {
        let error_string = "Couldn't write to a read only transaction";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxReadonly);
    }

    #[test]
    fn tx_condition_not_met() {
        let error_string = "Value being checked was not correct";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxConditionNotMet);
    }

    #[test]
    fn tx_key_already_exists() {
        let error_string = "The key being inserted already exists";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxKeyAlreadyExists);
    }

    #[test]
    fn tx_key_too_large() {
        let error_string = "Record id or key is too large";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxKeyTooLarge);
    }

    #[test]
    fn tx_value_too_large() {
        let error_string = "Record or value is too large";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxValueTooLarge);
    }

    #[test]
    fn tx_too_large() {
        let error_string = "Transaction is too large";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TxTooLarge);
    }

    #[test]
    fn ns_empty() {
        let error_string = "Specify a namespace to use";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NsEmpty);
    }

    #[test]
    fn db_empty() {
        let error_string = "Specify a database to use";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DbEmpty);
    }

    #[test]
    fn query_empty() {
        let error_string = "Specify some SQL code to execute";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::QueryEmpty);
    }

    #[test]
    fn query_remaining() {
        let error_string = "The SQL query was not parsed fully";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::QueryRemaining);
    }

    #[test]
    fn invalid_auth() {
        let error_string = "There was a problem with authentication";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidAuth);
    }

    #[test]
    fn unknown_auth() {
        let error_string = "Auth was expected to be set but was unknown";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::UnknownAuth);
    }

    #[test]
    fn invalid_query() {
        let error_string =
            "Parse error on line line number at character char when parsing 'string'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidQuery);
    }

    #[test]
    fn invalid_patch() {
        let error_string = "The JSON Patch contains invalid operations. error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidPatch);
    }

    #[test]
    fn patch_test() {
        let error_string =
        "Given test operation failed for JSON Patch. Expected `expected result`, but got `the issue` instead.";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::PatchTest);
    }

    #[test]
    fn http_disabled() {
        let error_string = "Remote HTTP request functions are not enabled";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::HttpDisabled);
    }

    #[test]
    fn invalid_param() {
        let error_string = "Found 'a name' but it is not possible to set a variable with this name";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidParam);
    }

    #[test]
    fn invalid_field() {
        let error_string = "Found 'a field' in SELECT clause on line line number, but field is not an aggregate function, and is not present in GROUP BY expression";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidField);
    }

    #[test]
    fn invalid_split() {
        let error_string = "Found 'a field' in SPLIT ON clause on line line number, but field is not present in SELECT expression";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidSplit);
    }

    #[test]
    fn invalid_order() {
        let error_string = "Found 'a field' in ORDER BY clause on line line number, but field is not present in SELECT expression";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidOrder);
    }

    #[test]
    fn invalid_group() {
        let error_string = "Found 'a field' in GROUP BY clause on line line number, but field is not present in SELECT expression";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidGroup);
    }

    #[test]
    fn invalid_limit() {
        let error_string =
            "Found a number but the LIMIT clause must evaluate to a positive integer";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidLimit);
    }

    #[test]
    fn invalid_start() {
        let error_string =
            "Found a number but the START clause must evaluate to a positive integer";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidStart);
    }

    #[test]
    fn invalid_script() {
        let error_string = "Problem with embedded script function. error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidScript);
    }

    #[test]
    fn invalid_function() {
        let error_string =
            "There was a problem running the function_name() function. error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidFunction);
    }

    #[test]
    fn invalid_arguments() {
        let error_string = "Incorrect arguments for function function_name(). error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidArguments);
    }

    #[test]
    fn invalid_url() {
        let error_string = "The URL `an invalid url` is invalid";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidUrl);
    }

    #[test]
    fn query_timedout() {
        let error_string = "The query was not executed because it exceeded the timeout";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::QueryTimedout);
    }

    #[test]
    fn query_cancelled() {
        let error_string = "The query was not executed due to a cancelled transaction";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::QueryCancelled);
    }

    #[test]
    fn query_not_executed() {
        let error_string = "The query was not executed due to a failed transaction";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::QueryNotExecuted);
    }

    #[test]
    fn query_not_executed_detail() {
        let error_string = "The query was not executed due to a failed transaction. error message";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::QueryNotExecutedDetail);
    }

    #[test]
    fn ns_not_allowed() {
        let error_string =
            "You don't have permission to change to the unchangable_namespace namespace";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NsNotAllowed);
    }

    #[test]
    fn db_not_allowed() {
        let error_string =
            "You don't have permission to change to the unchangable_database database";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DbNotAllowed);
    }

    #[test]
    fn ns_not_found() {
        let error_string = "The namespace 'nonexistent_namespace' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NsNotFound);
    }

    #[test]
    fn nt_not_found() {
        let error_string = "The namespace token 'a token' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NtNotFound);
    }

    #[test]
    fn nl_not_found() {
        let error_string = "The namespace login 'a login' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NlNotFound);
    }

    #[test]
    fn db_not_found() {
        let error_string = "The database 'a database' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DbNotFound);
    }

    #[test]
    fn dt_not_found() {
        let error_string = "The database token 'a token' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DtNotFound);
    }

    #[test]
    fn dl_not_found() {
        let error_string = "The database login 'a login' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DlNotFound);
    }

    #[test]
    fn fc_not_found() {
        let error_string = "The function 'fn::function_name' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::FcNotFound);
    }

    #[test]
    fn sc_not_found() {
        let error_string = "The scope 'a scope' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::ScNotFound);
    }

    #[test]
    fn cl_already_exists() {
        let error_string = "The node 'a node' already exists";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::ClAlreadyExists);
    }

    #[test]
    fn nd_not_found() {
        let error_string = "The node 'a node' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NdNotFound);
    }

    #[test]
    fn st_not_found() {
        let error_string = "The scope token 'scope_token' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::StNotFound);
    }

    #[test]
    fn pa_not_found() {
        let error_string = "The param '$invalid_param' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::PaNotFound);
    }

    #[test]
    fn tb_not_found() {
        let error_string = "The table 'invalid_table' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TbNotFound);
    }

    #[test]
    fn lv_not_found() {
        let error_string = "The live query 'a_query' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::LvNotFound);
    }

    #[test]
    fn lq_not_found() {
        let error_string = "The cluster live query 'a live query' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::LqNotFound);
    }

    #[test]
    fn az_not_found() {
        let error_string = "The analyzer 'invalid\'analyzer' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::AzNotFound);
    }

    #[test]
    fn ix_not_found() {
        let error_string = "The index '0001' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::IxNotFound);
    }

    #[test]
    fn user_root_not_found() {
        let error_string = "The root user 'root_user' does not exist";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::UserRootNotFound);
    }

    #[test]
    fn user_ns_not_found() {
        let error_string = "The user 'a_user' does not exist in the namespace 'a namespace'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::UserNsNotFound);
    }

    #[test]
    fn user_db_not_found() {
        let error_string = "The user 'a user' does not exist in the database 'invalid'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::UserDbNotFound);
    }

    #[test]
    fn realtime_disabled() {
        let error_string = "Unable to perform the realtime query";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::RealtimeDisabled);
    }

    #[test]
    fn computation_depth_exceeded() {
        let error_string =
            "Reached excessive computation depth due to functions, subqueries, or futures";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::ComputationDepthExceeded);
    }

    #[test]
    fn invalid_statement_target() {
        let error_string = "Can not execute statement using value '$32wsd44'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidStatementTarget);
    }

    #[test]
    fn create_statement() {
        let error_string = "Can not execute CREATE statement using value 'value'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::CreateStatement);
    }

    #[test]
    fn update_statement() {
        let error_string = "Can not execute UPDATE statement using value '#4cc'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::UpdateStatement);
    }

    #[test]
    fn relate_statement() {
        let error_string = "Can not execute RELATE statement using value '\"|'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::RelateStatement);
    }

    #[test]
    fn delete_statement() {
        let error_string = "Can not execute DELETE statement using value 'a value'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DeleteStatement);
    }

    #[test]
    fn insert_statement() {
        let error_string = "Can not execute INSERT statement using value 'a value'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InsertStatement);
    }

    #[test]
    fn live_statement() {
        let error_string = "Can not execute LIVE statement using value 'a value'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::LiveStatement);
    }

    #[test]
    fn kill_statement() {
        let error_string = "Can not execute KILL statement using id '01'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::KillStatement);
    }

    #[test]
    fn table_permissions() {
        let error_string = "You don't have permission to run this query on the `slkr` table";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TablePermissions);
    }

    #[test]
    fn table_is_view() {
        let error_string = "Unable to write to the `478eussd` table while setup as a view";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TableIsView);
    }

    #[test]
    fn record_exists() {
        let error_string = "Database record `res8as7dvu` already exists";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::RecordExists);
    }

    #[test]
    fn index_exists() {
        let error_string =
            "Database index `as;dliru` already contains sakdlru, with record `sadli4r7u:alskdur`";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::IndexExists);
    }

    #[test]
    fn field_check() {
        let error_string = "Found aslkdu4rlasjdv for field `asldkjrfr`, with record `rjurikuie`, but expected a 4789sasjfv:klrudsam";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::FieldCheck);
    }

    #[test]
    fn field_value() {
        let error_string =
            "Found sdlkrukavm for field `847aasldjvla`, with record `aiowe74as;y`, but field must conform to: aslki47lkscv;m:45eiuai8";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::FieldValue);
    }

    #[test]
    fn id_mismatch() {
        let error_string =
            "Found :sliduarorjc for the id field, but a specific record has been specified";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::IdMismatch);
    }

    #[test]
    fn id_invalid() {
        let error_string = "Found aslikd47asljdkv for the Record ID but this is not a valid id";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::IdInvalid);
    }

    #[test]
    fn coerce_to() {
        let error_string = "Expected a asiole47sckjl but found lskad74cklm";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::CoerceTo);
    }

    #[test]
    fn convert_to() {
        let error_string =
            "Expected a asildu;4ids7 but cannot convert lak;sd74csjhl into a alksid74*jf238764$%^";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::ConvertTo);
    }

    #[test]
    fn length_invalid() {
        let error_string = "Expected a 7as4lksd but the array had q38247q9 items";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::LengthInvalid);
    }

    #[test]
    fn try_add() {
        let error_string = "Cannot perform addition with '89423ajd' and 'q9328paslu;d'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TryAdd);
    }

    #[test]
    fn try_sub() {
        let error_string = "Cannot perform subtraction with 'q9823sd;ljkjf' and 'aosie674sdu'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TrySub);
    }

    #[test]
    fn try_mul() {
        let error_string = "Cannot perform multiplication with 'ao8we4sdjl' and 'aoisd6jcsh'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TryMul);
    }

    #[test]
    fn try_div() {
        let error_string = "Cannot perform division with 'ajsldh4oi6' and 'alkxc;a7i4'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TryDiv);
    }

    #[test]
    fn try_pow() {
        let error_string = "Cannot raise the value 'als;id74cslk' with 'alskd74'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TryPow);
    }

    #[test]
    fn try_neg() {
        let error_string = "Cannot negate the value 'aslkcu4i'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TryNeg);
    }

    #[test]
    fn try_from() {
        let error_string = "Cannot convert from 'askdjl4iso7' to 'aiowse47scl'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TryFrom);
    }

    #[test]
    fn http() {
        let error_string = "There was an error processing a remote HTTP request: ;oia47wed";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Http);
    }

    #[test]
    fn channel() {
        let error_string = "There was an error processing a value in parallel: oi57cmv";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Channel);
    }

    #[test]
    fn io() {
        let error_string = "I/O error: lkas;d745";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Io);
    }

    #[test]
    fn encode() {
        let error_string = "Key encoding error: 5io7d";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Encode);
    }

    #[test]
    fn decode() {
        let error_string = "Key decoding error: alkse7u47";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Decode);
    }

    #[test]
    fn revision() {
        let error_string = "Versioned error: zlks;du4rc";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Revision);
    }

    #[test]
    fn corrupted_index() {
        let error_string = "Index is corrupted";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::CorruptedIndex);
    }

    #[test]
    fn no_index_found_for_match() {
        let error_string =
            "There was no suitable full-text index supporting the expression 'zlxk;cu5t'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NoIndexFoundForMatch);
    }

    #[test]
    fn analyzer_error() {
        let error_string = "A value can't be analyzed: ;litr7";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::AnalyzerError);
    }

    #[test]
    fn highlight_error() {
        let error_string = "A value can't be highlighted: 0c9sayur";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::HighlightError);
    }

    #[test]
    fn bincode() {
        let error_string = "Bincode error: 48e7d";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Bincode);
    }

    #[test]
    fn fst_error() {
        let error_string = "FstError error: 894573wo";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::FstError);
    }

    #[test]
    fn utf_8_error() {
        let error_string = "Utf8 error: 89ew74";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Utf8Error);
    }

    #[test]
    fn feature_not_yet_implemented() {
        let error_string = "Feature not yet implemented: 49786sa";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::FeatureNotYetImplemented);
    }

    #[test]
    fn duplicated_match_ref() {
        let error_string = "Duplicated Match reference: *";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::DuplicatedMatchRef);
    }

    #[test]
    fn timestamp_overflow() {
        let error_string = "Timestamp arithmetic error: 74d";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::TimestampOverflow);
    }

    #[test]
    fn internal() {
        let error_string = "Internal database error: ";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Internal);
    }

    #[test]
    fn unimplemented() {
        let error_string = "Unimplemented functionality: 587edj";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Unimplemented);
    }

    #[test]
    fn corrupted_versionstamp_in_key() {
        let error_string = "Versionstamp in key is corrupted: yhxzudro";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::CorruptedVersionstampInKey);
    }

    #[test]
    fn invalid_level() {
        let error_string = "Invalid level 's4x7i'";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::InvalidLevel);
    }

    #[test]
    fn iam_error() {
        let error_string = "IAM error: l8x4";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::IamError);
    }

    #[test]
    fn scripting_not_allowed() {
        let error_string = "Scripting functions are not allowed";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::ScriptingNotAllowed);
    }

    #[test]
    fn function_not_allowed() {
        let error_string = "Function 's4i7o' is not allowed to be executed";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::FunctionNotAllowed);
    }

    #[test]
    fn net_target_not_allowed() {
        let error_string = "Access to network target '8oe47' is not allowed";

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::NetTargetNotAllowed);
    }

    #[test]
    fn deprecated() {
        let error_string = ""; // This can match anything

        let error = QueryError::from_error_string(error_string);

        assert_eq!(error, QueryError::Deprecated);
    }
}
