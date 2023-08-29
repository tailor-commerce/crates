use str_pattern_macro::StrPattern;

#[derive(StrPattern, Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum QueryError {
    /// This error is used for ignoring a document when processing a query
    #[str_pattern("Conditional clause is not truthy")]
    Ignore,

    /// This error is used for breaking a loop in a foreach statement
    #[str_pattern("Break statement has been reached")]
    Break,

    /// This error is used for skipping a loop in a foreach statement
    #[str_pattern("Continue statement has been reached")]
    Continue,

    /// The database encountered unreachable logic
    #[str_pattern("The database encountered unreachable logic")]
    Unreachable,

    /// A custom error has been thrown
    #[str_pattern("An error occurred: {0}")]
    Thrown(String),

    /// There was a problem with the underlying datastore
    #[str_pattern("There was a problem with the underlying datastore: {0}")]
    Ds(String),

    /// There was a problem with a datastore transaction
    #[str_pattern("There was a problem with a datastore transaction: {0}")]
    Tx(String),

    /// There was an error when starting a new datastore transaction
    #[str_pattern("There was an error when starting a new datastore transaction")]
    TxFailure,

    /// The transaction was already cancelled or committed
    #[str_pattern("Couldn't update a finished transaction")]
    TxFinished,

    /// The current transaction was created as read-only
    #[str_pattern("Couldn't write to a read only transaction")]
    TxReadonly,

    /// The conditional value in the request was not equal
    #[str_pattern("Value being checked was not correct")]
    TxConditionNotMet,

    /// The key being inserted in the transaction already exists
    #[str_pattern("The key being inserted already exists")]
    TxKeyAlreadyExists,

    /// The key exceeds a limit set by the KV store
    #[str_pattern("Record id or key is too large")]
    TxKeyTooLarge,

    /// The value exceeds a limit set by the KV store
    #[str_pattern("Record or value is too large")]
    TxValueTooLarge,

    /// The transaction writes too much data for the KV store
    #[str_pattern("Transaction is too large")]
    TxTooLarge,

    /// No namespace has been selected
    #[str_pattern("Specify a namespace to use")]
    NsEmpty,

    /// No database has been selected
    #[str_pattern("Specify a database to use")]
    DbEmpty,

    /// No SQL query has been specified
    #[str_pattern("Specify some SQL code to execute")]
    QueryEmpty,

    /// There was an error with the SQL query
    #[str_pattern("The SQL query was not parsed fully")]
    QueryRemaining,

    /// There was an error with authentication
    #[str_pattern("There was a problem with authentication")]
    InvalidAuth,

    /// Auth was expected to be set but was unknown
    #[str_pattern("Auth was expected to be set but was unknown")]
    UnknownAuth,

    /// There was an error with the SQL query
    #[str_pattern("Parse error on line {line} at character {char} when parsing '{sql}'")]
    InvalidQuery {
        line: String,
        char: String,
        sql: String,
    },

    /// There was an error with the provided JSON Patch
    #[str_pattern("The JSON Patch contains invalid operations. {message}")]
    InvalidPatch { message: String },

    /// Given test operation failed for JSON Patch
    #[str_pattern("Given test operation failed for JSON Patch. Expected `{expected}`, but got `{got}` instead.")]
    PatchTest { expected: String, got: String },

    /// Remote HTTP request functions are not enabled
    #[str_pattern("Remote HTTP request functions are not enabled")]
    HttpDisabled,

    /// it is not possible to set a variable with the specified name
    #[str_pattern("Found '{name}' but it is not possible to set a variable with this name")]
    InvalidParam { name: String },

    #[str_pattern("Found '{field}' in SELECT clause on line {line}, but field is not an aggregate function, and is not present in GROUP BY expression")]
    InvalidField { line: String, field: String },

    #[str_pattern("Found '{field}' in SPLIT ON clause on line {line}, but field is not present in SELECT expression")]
    InvalidSplit { line: String, field: String },

    #[str_pattern("Found '{field}' in ORDER BY clause on line {line}, but field is not present in SELECT expression")]
    InvalidOrder { line: String, field: String },

    #[str_pattern("Found '{field}' in GROUP BY clause on line {line}, but field is not present in SELECT expression")]
    InvalidGroup { line: String, field: String },

    /// The LIMIT clause must evaluate to a positive integer
    #[str_pattern("Found {value} but the LIMIT clause must evaluate to a positive integer")]
    InvalidLimit { value: String },

    /// The START clause must evaluate to a positive integer
    #[str_pattern("Found {value} but the START clause must evaluate to a positive integer")]
    InvalidStart { value: String },

    /// There was an error with the provided JavaScript code
    #[str_pattern("Problem with embedded script function. {message}")]
    InvalidScript { message: String },

    /// There was a problem running the specified function
    #[str_pattern("There was a problem running the {name}() function. {message}")]
    InvalidFunction { name: String, message: String },

    /// The wrong quantity or magnitude of arguments was given for the specified function
    #[str_pattern("Incorrect arguments for function {name}(). {message}")]
    InvalidArguments { name: String, message: String },

    /// The URL is invalid
    #[str_pattern("The URL `{0}` is invalid")]
    InvalidUrl(String),

    /// The query timedout
    #[str_pattern("The query was not executed because it exceeded the timeout")]
    QueryTimedout,

    /// The query did not execute, because the transaction was cancelled
    #[str_pattern("The query was not executed due to a cancelled transaction")]
    QueryCancelled,

    /// The query did not execute, because the transaction has failed
    #[str_pattern("The query was not executed due to a failed transaction")]
    QueryNotExecuted,

    /// The query did not execute, because the transaction has failed (with a message)
    #[str_pattern("The query was not executed due to a failed transaction. {message}")]
    QueryNotExecutedDetail { message: String },

    /// The permissions do not allow for changing to the specified namespace
    #[str_pattern("You don't have permission to change to the {ns} namespace")]
    NsNotAllowed { ns: String },

    /// The permissions do not allow for changing to the specified database
    #[str_pattern("You don't have permission to change to the {db} database")]
    DbNotAllowed { db: String },

    /// The requested namespace does not exist
    #[str_pattern("The namespace '{value}' does not exist")]
    NsNotFound { value: String },

    /// The requested namespace token does not exist
    #[str_pattern("The namespace token '{value}' does not exist")]
    NtNotFound { value: String },

    /// The requested namespace login does not exist
    #[str_pattern("The namespace login '{value}' does not exist")]
    NlNotFound { value: String },

    /// The requested database does not exist
    #[str_pattern("The database '{value}' does not exist")]
    DbNotFound { value: String },

    /// The requested database token does not exist
    #[str_pattern("The database token '{value}' does not exist")]
    DtNotFound { value: String },

    /// The requested database login does not exist
    #[str_pattern("The database login '{value}' does not exist")]
    DlNotFound { value: String },

    /// The requested function does not exist
    #[str_pattern("The function 'fn::{value}' does not exist")]
    FcNotFound { value: String },

    /// The requested scope does not exist
    #[str_pattern("The scope '{value}' does not exist")]
    ScNotFound { value: String },

    // The cluster node already exists
    #[str_pattern("The node '{value}' already exists")]
    ClAlreadyExists { value: String },

    // The cluster node does not exist
    #[str_pattern("The node '{value}' does not exist")]
    NdNotFound { value: String },

    /// The requested scope token does not exist
    #[str_pattern("The scope token '{value}' does not exist")]
    StNotFound { value: String },

    /// The requested param does not exist
    #[str_pattern("The param '${value}' does not exist")]
    PaNotFound { value: String },

    /// The requested table does not exist
    #[str_pattern("The table '{value}' does not exist")]
    TbNotFound { value: String },

    /// The requested live query does not exist
    #[str_pattern("The live query '{value}' does not exist")]
    LvNotFound { value: String },

    /// The requested cluster live query does not exist
    #[str_pattern("The cluster live query '{value}' does not exist")]
    LqNotFound { value: String },

    /// The requested analyzer does not exist
    #[str_pattern("The analyzer '{value}' does not exist")]
    AzNotFound { value: String },

    /// The requested analyzer does not exist
    #[str_pattern("The index '{value}' does not exist")]
    IxNotFound { value: String },

    /// The requested root user does not exist
    #[str_pattern("The root user '{value}' does not exist")]
    UserRootNotFound { value: String },

    /// The requested namespace user does not exist
    #[str_pattern("The user '{value}' does not exist in the namespace '{ns}'")]
    UserNsNotFound { value: String, ns: String },

    /// The requested database user does not exist
    #[str_pattern("The user '{value}' does not exist in the database '{db}'")]
    UserDbNotFound { value: String, db: String },

    /// Unable to perform the realtime query
    #[str_pattern("Unable to perform the realtime query")]
    RealtimeDisabled,

    /// Reached excessive computation depth due to functions, subqueries, or futures
    #[str_pattern("Reached excessive computation depth due to functions, subqueries, or futures")]
    ComputationDepthExceeded,

    /// Can not execute statement using the specified value
    #[str_pattern("Can not execute statement using value '{value}'")]
    InvalidStatementTarget { value: String },

    /// Can not execute CREATE statement using the specified value
    #[str_pattern("Can not execute CREATE statement using value '{value}'")]
    CreateStatement { value: String },

    /// Can not execute UPDATE statement using the specified value
    #[str_pattern("Can not execute UPDATE statement using value '{value}'")]
    UpdateStatement { value: String },

    /// Can not execute RELATE statement using the specified value
    #[str_pattern("Can not execute RELATE statement using value '{value}'")]
    RelateStatement { value: String },

    /// Can not execute DELETE statement using the specified value
    #[str_pattern("Can not execute DELETE statement using value '{value}'")]
    DeleteStatement { value: String },

    /// Can not execute INSERT statement using the specified value
    #[str_pattern("Can not execute INSERT statement using value '{value}'")]
    InsertStatement { value: String },

    /// Can not execute LIVE statement using the specified value
    #[str_pattern("Can not execute LIVE statement using value '{value}'")]
    LiveStatement { value: String },

    /// Can not execute KILL statement using the specified id
    #[str_pattern("Can not execute KILL statement using id '{value}'")]
    KillStatement { value: String },

    /// The permissions do not allow this query to be run on this table
    #[str_pattern("You don't have permission to run this query on the `{table}` table")]
    TablePermissions { table: String },

    /// The specified table can not be written as it is setup as a foreign table view
    #[str_pattern("Unable to write to the `{table}` table while setup as a view")]
    TableIsView { table: String },

    /// A database entry for the specified record already exists
    #[str_pattern("Database record `{thing}` already exists")]
    RecordExists { thing: String },

    /// A database index entry for the specified record already exists
    #[str_pattern("Database index `{index}` already contains {value}, with record `{thing}`")]
    IndexExists {
        thing: String,
        index: String,
        value: String,
    },

    /// The specified field did not conform to the field type check
    #[str_pattern(
        "Found {value} for field `{field}`, with record `{thing}`, but expected a {check}"
    )]
    FieldCheck {
        thing: String,
        value: String,
        field: String,
        check: String,
    },

    /// The specified field did not conform to the field ASSERT clause
    #[str_pattern("Found {value} for field `{field}`, with record `{thing}`, but field must conform to: {check}")]
    FieldValue {
        thing: String,
        value: String,
        field: String,
        check: String,
    },

    /// Found a record id for the record but we are creating a specific record
    #[str_pattern("Found {value} for the id field, but a specific record has been specified")]
    IdMismatch { value: String },

    /// Found a record id for the record but this is not a valid id
    #[str_pattern("Found {value} for the Record ID but this is not a valid id")]
    IdInvalid { value: String },

    /// Unable to coerce to a value to another value
    #[str_pattern("Expected a {into} but found {from}")]
    CoerceTo { from: String, into: String },

    /// Unable to convert a value to another value
    #[str_pattern("Expected a {into} but cannot convert {from} into a {into}")]
    ConvertTo { from: String, into: String },

    /// Unable to coerce to a value to another value
    #[str_pattern("Expected a {kind} but the array had {size} items")]
    LengthInvalid { kind: String, size: String },

    /// Cannot perform addition
    #[str_pattern("Cannot perform addition with '{0}' and '{1}'")]
    TryAdd(String, String),

    /// Cannot perform subtraction
    #[str_pattern("Cannot perform subtraction with '{0}' and '{1}'")]
    TrySub(String, String),

    /// Cannot perform multiplication
    #[str_pattern("Cannot perform multiplication with '{0}' and '{1}'")]
    TryMul(String, String),

    /// Cannot perform division
    #[str_pattern("Cannot perform division with '{0}' and '{1}'")]
    TryDiv(String, String),

    /// Cannot perform power
    #[str_pattern("Cannot raise the value '{0}' with '{1}'")]
    TryPow(String, String),

    /// Cannot perform negation
    #[str_pattern("Cannot negate the value '{0}'")]
    TryNeg(String),

    /// It's is not possible to convert between the two types
    #[str_pattern("Cannot convert from '{0}' to '{1}'")]
    TryFrom(String, String),

    /// There was an error processing a remote HTTP request
    #[str_pattern("There was an error processing a remote HTTP request: {0}")]
    Http(String),

    /// There was an error processing a value in parallel
    #[str_pattern("There was an error processing a value in parallel: {0}")]
    Channel(String),

    /// Represents an underlying error with IO encoding / decoding
    #[str_pattern("I/O error: {0}")]
    Io(String),

    /// Represents an error when encoding a key-value entry
    #[str_pattern("Key encoding error: {0}")]
    Encode(String),

    /// Represents an error when decoding a key-value entry
    #[str_pattern("Key decoding error: {0}")]
    Decode(String),

    /// Represents an underlying error with versioned data encoding / decoding
    #[str_pattern("Versioned error: {0}")]
    Revision(String),

    /// The index has been found to be inconsistent
    #[str_pattern("Index is corrupted")]
    CorruptedIndex,

    /// The query planner did not find an index able to support the match @@ operator on a given expression
    #[str_pattern("There was no suitable full-text index supporting the expression '{value}'")]
    NoIndexFoundForMatch { value: String },

    /// Represents an error when analyzing a value
    #[str_pattern("A value can't be analyzed: {0}")]
    AnalyzerError(String),

    /// Represents an error when trying to highlight a value
    #[str_pattern("A value can't be highlighted: {0}")]
    HighlightError(String),

    /// Represents an underlying error with Bincode serializing / deserializing
    #[str_pattern("Bincode error: {0}")]
    Bincode(String),

    /// Represents an underlying error with FST
    #[str_pattern("FstError error: {0}")]
    FstError(String),

    /// Represents an underlying error while reading UTF8 characters
    #[str_pattern("Utf8 error: {0}")]
    Utf8Error(String),

    /// The feature has not yet being implemented
    #[str_pattern("Feature not yet implemented: {feature}")]
    FeatureNotYetImplemented { feature: String },

    /// Duplicated match references are not allowed
    #[str_pattern("Duplicated Match reference: {mr}")]
    DuplicatedMatchRef { mr: String },

    /// Represents a failure in timestamp arithmetic related to database internals
    #[str_pattern("Timestamp arithmetic error: {0}")]
    TimestampOverflow(String),

    /// Internal server error
    /// This should be used extremely sporadically, since we lose the type of error as a consequence
    /// There will be times when it is useful, such as with unusual type conversion errors
    #[str_pattern("Internal database error: {0}")]
    Internal(String),

    /// Unimplemented functionality
    #[str_pattern("Unimplemented functionality: {0}")]
    Unimplemented(String),

    #[str_pattern("Versionstamp in key is corrupted: {0}")]
    CorruptedVersionstampInKey(String),

    /// Invalid level
    #[str_pattern("Invalid level '{0}'")]
    InvalidLevel(String),

    /// Represents an underlying IAM error
    #[str_pattern("IAM error: {0}")]
    IamError(String),

    //
    // Capabilities
    //
    /// Scripting is not allowed
    #[str_pattern("Scripting functions are not allowed")]
    ScriptingNotAllowed,

    /// Function is not allowed
    #[str_pattern("Function '{0}' is not allowed to be executed")]
    FunctionNotAllowed(String),

    /// Network target is not allowed
    #[str_pattern("Access to network target '{0}' is not allowed")]
    NetTargetNotAllowed(String),

    /// Statement has been deprecated
    #[str_pattern("{0}")]
    Deprecated(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore() {
        let error_string = "Conditional clause is not truthy";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::Ignore);
    }

    #[test]
    fn r#break() {
        let error_string = "Break statement has been reached";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::Break);
    }

    #[test]
    fn r#continue() {
        let error_string = "Continue statement has been reached";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::Continue);
    }

    #[test]
    fn unreachable() {
        let error_string = "The database encountered unreachable logic";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::Unreachable);
    }

    #[test]
    fn thrown() {
        let message = "error message".to_string();

        let error_string = format!("An error occurred: {}", message);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Thrown(message));
    }

    #[test]
    fn ds() {
        let message = "error message".to_string();

        let error_string = format!(
            "There was a problem with the underlying datastore: {}",
            message
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Ds(message));
    }

    #[test]
    fn tx() {
        let message = "error message".to_string();

        let error_string = format!(
            "There was a problem with a datastore transaction: {}",
            message
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Tx(message));
    }

    #[test]
    fn tx_failure() {
        let error_string = "There was an error when starting a new datastore transaction";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxFailure);
    }

    #[test]
    fn tx_finished() {
        let error_string = "Couldn't update a finished transaction";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxFinished);
    }

    #[test]
    fn tx_readonly() {
        let error_string = "Couldn't write to a read only transaction";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxReadonly);
    }

    #[test]
    fn tx_condition_not_met() {
        let error_string = "Value being checked was not correct";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxConditionNotMet);
    }

    #[test]
    fn tx_key_already_exists() {
        let error_string = "The key being inserted already exists";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxKeyAlreadyExists);
    }

    #[test]
    fn tx_key_too_large() {
        let error_string = "Record id or key is too large";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxKeyTooLarge);
    }

    #[test]
    fn tx_value_too_large() {
        let error_string = "Record or value is too large";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxValueTooLarge);
    }

    #[test]
    fn tx_too_large() {
        let error_string = "Transaction is too large";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::TxTooLarge);
    }

    #[test]
    fn ns_empty() {
        let error_string = "Specify a namespace to use";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::NsEmpty);
    }

    #[test]
    fn db_empty() {
        let error_string = "Specify a database to use";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::DbEmpty);
    }

    #[test]
    fn query_empty() {
        let error_string = "Specify some SQL code to execute";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::QueryEmpty);
    }

    #[test]
    fn query_remaining() {
        let error_string = "The SQL query was not parsed fully";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::QueryRemaining);
    }

    #[test]
    fn invalid_auth() {
        let error_string = "There was a problem with authentication";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::InvalidAuth);
    }

    #[test]
    fn unknown_auth() {
        let error_string = "Auth was expected to be set but was unknown";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::UnknownAuth);
    }

    #[test]
    fn invalid_query() {
        let line = "line".to_string();
        let char = "char".to_string();
        let sql = "sql query".to_string();

        let error_string = format!(
            "Parse error on line {line} at character {char} when parsing '{sql}'",
            line = line,
            char = char,
            sql = sql
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidQuery { line, char, sql });
    }

    #[test]
    fn invalid_patch() {
        let message = "unexpected ',' at line 1 column 2".to_string();

        let error_string = format!("The JSON Patch contains invalid operations. {}", message);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidPatch { message });
    }

    #[test]
    fn patch_test() {
        let expected = "expected result".to_string();
        let got = "the issue".to_string();

        let error_string =
              format!(
                  "Given test operation failed for JSON Patch. Expected `{expected}`, but got `{got}` instead.",
                  expected = expected, got = got);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::PatchTest { expected, got });
    }

    #[test]
    fn http_disabled() {
        let error_string = "Remote HTTP request functions are not enabled";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::HttpDisabled);
    }

    #[test]
    fn invalid_param() {
        let name = "a name".to_string();

        let error_string = format!(
            "Found '{name}' but it is not possible to set a variable with this name",
            name = name
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidParam { name });
    }

    #[test]
    fn invalid_field() {
        let line = "89".to_string();
        let field = "a field".to_string();

        let error_string = format!("Found '{field}' in SELECT clause on line {line}, but field is not an aggregate function, and is not present in GROUP BY expression", field = field, line = line);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidField { line, field });
    }

    #[test]
    fn invalid_split() {
        let line = "1000".to_string();
        let field = "name".to_string();

        let error_string = format!("Found '{field}' in SPLIT ON clause on line {line}, but field is not present in SELECT expression", field = field, line = line);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidSplit { line, field });
    }

    #[test]
    fn invalid_order() {
        let line = "301".to_string();
        let field = "a field".to_string();

        let error_string = format!("Found '{field}' in ORDER BY clause on line {line}, but field is not present in SELECT expression", line = line, field = field);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidOrder { line, field });
    }

    #[test]
    fn invalid_group() {
        let line = "1000".to_string();
        let field = "name".to_string();

        let error_string = format!("Found '{field}' in GROUP BY clause on line {line}, but field is not present in SELECT expression", line = line, field = field);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidGroup { line, field });
    }

    #[test]
    fn invalid_limit() {
        let value = "-1".to_string();

        let error_string = format!(
            "Found {value} but the LIMIT clause must evaluate to a positive integer",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidLimit { value });
    }

    #[test]
    fn invalid_start() {
        let value = "20.1".to_string();

        let error_string = format!(
            "Found {value} but the START clause must evaluate to a positive integer",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidStart { value });
    }

    #[test]
    fn invalid_script() {
        let message = "error message".to_string();

        let error_string = format!(
            "Problem with embedded script function. {message}",
            message = message
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidScript { message });
    }

    #[test]
    fn invalid_function() {
        let message = "error message".to_string();
        let name = "function_name".to_string();

        let error_string = format!(
            "There was a problem running the {name}() function. {message}",
            message = message,
            name = name
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidFunction { name, message });
    }

    #[test]
    fn invalid_arguments() {
        let name = "function_name".to_string();
        let message = "error message".to_string();

        let error_string = format!(
            "Incorrect arguments for function {name}(). {message}",
            name = name,
            message = message
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidArguments { name, message });
    }

    #[test]
    fn invalid_url() {
        let url = "an invalid url".to_string();

        let error_string = format!("The URL `{url}` is invalid", url = url);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidUrl(url));
    }

    #[test]
    fn query_timedout() {
        let error_string = "The query was not executed because it exceeded the timeout";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::QueryTimedout);
    }

    #[test]
    fn query_cancelled() {
        let error_string = "The query was not executed due to a cancelled transaction";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::QueryCancelled);
    }

    #[test]
    fn query_not_executed() {
        let error_string = "The query was not executed due to a failed transaction";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::QueryNotExecuted);
    }

    #[test]
    fn query_not_executed_detail() {
        let message = "error message".to_string();

        let error_string = format!(
            "The query was not executed due to a failed transaction. {message}",
            message = message
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::QueryNotExecutedDetail { message });
    }

    #[test]
    fn ns_not_allowed() {
        let ns = "unchangable_namespace".to_string();

        let error_string = format!(
            "You don't have permission to change to the {ns} namespace",
            ns = ns
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NsNotAllowed { ns });
    }

    #[test]
    fn db_not_allowed() {
        let db = "unchangable_database".to_string();

        let error_string = format!(
            "You don't have permission to change to the {db} database",
            db = db
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::DbNotAllowed { db });
    }

    #[test]
    fn ns_not_found() {
        let value = "nonexistent_namespace".to_string();

        let error_string = format!("The namespace '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NsNotFound { value });
    }

    #[test]
    fn nt_not_found() {
        let value = "a token".to_string();

        let error_string = format!(
            "The namespace token '{value}' does not exist",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NtNotFound { value });
    }

    #[test]
    fn nl_not_found() {
        let value = "a login".to_string();

        let error_string = format!(
            "The namespace login '{value}' does not exist",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NlNotFound { value });
    }

    #[test]
    fn db_not_found() {
        let value = "a database".to_string();

        let error_string = format!("The database '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::DbNotFound { value });
    }

    #[test]
    fn dt_not_found() {
        let value = "a token".to_string();

        let error_string = format!("The database token '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::DtNotFound { value });
    }

    #[test]
    fn dl_not_found() {
        let value = "a login".to_string();

        let error_string = format!("The database login '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::DlNotFound { value });
    }

    #[test]
    fn fc_not_found() {
        let value = "function_name".to_string();

        let error_string = format!("The function 'fn::{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::FcNotFound { value });
    }

    #[test]
    fn sc_not_found() {
        let value = "a scope".to_string();

        let error_string = format!("The scope '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::ScNotFound { value });
    }

    #[test]
    fn cl_already_exists() {
        let value = "a node".to_string();

        let error_string = format!("The node '{value}' already exists", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::ClAlreadyExists { value });
    }

    #[test]
    fn nd_not_found() {
        let value = "a node".to_string();

        let error_string = format!("The node '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NdNotFound { value });
    }

    #[test]
    fn st_not_found() {
        let value = "scope_token".to_string();

        let error_string = format!("The scope token '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::StNotFound { value });
    }

    #[test]
    fn pa_not_found() {
        let value = "invalid_param".to_string();

        let error_string = format!("The param '${value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::PaNotFound { value });
    }

    #[test]
    fn tb_not_found() {
        let value = "invalid_table".to_string();

        let error_string = format!("The table '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TbNotFound { value });
    }

    #[test]
    fn lv_not_found() {
        let value = "a live query".to_string();

        let error_string = format!("The live query '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::LvNotFound { value });
    }

    #[test]
    fn lq_not_found() {
        let value = "a live query".to_string();

        let error_string = format!(
            "The cluster live query '{value}' does not exist",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::LqNotFound { value });
    }

    #[test]
    fn az_not_found() {
        let value = "invalid'analyzer".to_string();

        let error_string = format!("The analyzer '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::AzNotFound { value });
    }

    #[test]
    fn ix_not_found() {
        let value = "0001".to_string();

        let error_string = format!("The index '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::IxNotFound { value });
    }

    #[test]
    fn user_root_not_found() {
        let value = "root_user".to_string();

        let error_string = format!("The root user '{value}' does not exist", value = value);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::UserRootNotFound { value });
    }

    #[test]
    fn user_ns_not_found() {
        let value = "a_user".to_string();
        let ns = "a namespace".to_string();

        let error_string = format!(
            "The user '{value}' does not exist in the namespace '{ns}'",
            value = value,
            ns = ns
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::UserNsNotFound { value, ns });
    }

    #[test]
    fn user_db_not_found() {
        let value = "a user".to_string();
        let db = "invalid".to_string();

        let error_string = format!(
            "The user '{value}' does not exist in the database '{db}'",
            value = value,
            db = db
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::UserDbNotFound { value, db });
    }

    #[test]
    fn realtime_disabled() {
        let error_string = "Unable to perform the realtime query";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::RealtimeDisabled);
    }

    #[test]
    fn computation_depth_exceeded() {
        let error_string =
            "Reached excessive computation depth due to functions, subqueries, or futures";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::ComputationDepthExceeded);
    }

    #[test]
    fn invalid_statement_target() {
        let value = "$32wsd44".to_string();

        let error_string = format!(
            "Can not execute statement using value '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidStatementTarget { value });
    }

    #[test]
    fn create_statement() {
        let value = "value".to_string();

        let error_string = format!(
            "Can not execute CREATE statement using value '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::CreateStatement { value });
    }

    #[test]
    fn update_statement() {
        let value = "#4cc".to_string();

        let error_string = format!(
            "Can not execute UPDATE statement using value '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::UpdateStatement { value });
    }

    #[test]
    fn relate_statement() {
        let value = "\"|".to_string();

        let error_string = format!("Can not execute RELATE statement using value '{value}'");

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::RelateStatement { value });
    }

    #[test]
    fn delete_statement() {
        let value = "a value".to_string();

        let error_string = format!(
            "Can not execute DELETE statement using value '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::DeleteStatement { value });
    }

    #[test]
    fn insert_statement() {
        let value = "a value".to_string();

        let error_string = format!(
            "Can not execute INSERT statement using value '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InsertStatement { value });
    }

    #[test]
    fn live_statement() {
        let value = "a value".to_string();

        let error_string = format!(
            "Can not execute LIVE statement using value '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::LiveStatement { value });
    }

    #[test]
    fn kill_statement() {
        let value = "01".to_string();

        let error_string = format!(
            "Can not execute KILL statement using id '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::KillStatement { value });
    }

    #[test]
    fn table_permissions() {
        let table = "slkr".to_owned();

        let error_string = format!(
            "You don't have permission to run this query on the `{table}` table",
            table = table
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TablePermissions { table });
    }

    #[test]
    fn table_is_view() {
        let table = "478eussd".to_owned();

        let error_string = format!(
            "Unable to write to the `{table}` table while setup as a view",
            table = table
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TableIsView { table });
    }

    #[test]
    fn record_exists() {
        let thing = "res8as7dvu".to_owned();

        let error_string = format!("Database record `{thing}` already exists", thing = thing);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::RecordExists { thing });
    }

    #[test]
    fn index_exists() {
        let index = "as;dliru".to_owned();
        let value = "sakdlru".to_owned();
        let thing = "sadli4r7u:alskdur".to_owned();

        let error_string = format!(
            "Database index `{index}` already contains {value}, with record `{thing}`",
            index = &index,
            value = &value,
            thing = &thing
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(
            error,
            QueryError::IndexExists {
                index,
                value,
                thing
            }
        );
    }

    #[test]
    fn field_check() {
        let value = "aslkdu4rlasjdv".to_owned();
        let field = "rjurikuie".to_owned();
        let thing = "asldkjrfr".to_owned();
        let check = "4789sasjfv:klrudsam".to_owned();

        let error_string = format!(
            "Found {value} for field `{field}`, with record `{thing}`, but expected a {check}",
            value = value,
            thing = thing,
            field = field,
            check = check
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(
            error,
            QueryError::FieldCheck {
                value,
                field,
                thing,
                check
            }
        );
    }

    #[test]
    fn field_value() {
        let value = "aslkdu4rlasjdv".to_string();
        let field = "847aasldjvla".to_string();
        let thing = "aiowe74as;y".to_string();
        let check = "aslki47lkscv;m:45eiuai8".to_string();

        let error_string =
                  format!("Found {value} for field `{field}`, with record `{thing}`, but field must conform to: {check}", value = value, thing = thing, field = field, check = check);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(
            error,
            QueryError::FieldValue {
                thing,
                value,
                field,
                check
            }
        );
    }

    #[test]
    fn id_mismatch() {
        let value = ":sliduarorjc".to_string();

        let error_string = format!(
            "Found {value} for the id field, but a specific record has been specified",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::IdMismatch { value });
    }

    #[test]
    fn id_invalid() {
        let value = "aslikd47asljdkv".to_string();

        let error_string = format!(
            "Found {value} for the Record ID but this is not a valid id",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::IdInvalid { value });
    }

    #[test]
    fn coerce_to() {
        let from = "aslkdu4rlasjdv".to_string();
        let into = "847aasldjvla".to_string();

        let error_string = format!(
            "Expected a {into} but found {from}",
            from = from,
            into = into
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::CoerceTo { from, into });
    }

    #[test]
    fn convert_to() {
        let into = "aslkdu4rlasjdv".to_string();
        let from = "847aasldjvla".to_string();
        let into_2 = "alksid74*jf238764$%^".to_string();

        let error_string = format!(
            "Expected a {into} but cannot convert {from} into a {into_2}",
            from = from,
            into = into,
            into_2 = into_2
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::ConvertTo { from, into });
    }

    #[test]
    fn length_invalid() {
        let kind = "7as4lksd".to_string();
        let size = "q38247q9".to_string();

        let error_string = format!(
            "Expected a {kind} but the array had {size} items",
            kind = kind,
            size = size
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::LengthInvalid { kind, size });
    }

    #[test]
    fn try_add() {
        let first = "89423ajd".to_string();
        let second = "q9328paslu;d".to_string();

        let error_string = format!(
            "Cannot perform addition with '{first}' and '{second}'",
            first = first,
            second = second
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TryAdd(first, second));
    }

    #[test]
    fn try_sub() {
        let first = "q9823sd;ljkjf".to_string();
        let second = "aosiue674sdu".to_string();

        let error_string = format!(
            "Cannot perform subtraction with '{first}' and '{second}'",
            first = first,
            second = second
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TrySub(first, second));
    }

    #[test]
    fn try_mul() {
        let first = "ao8we4sdjl".to_string();
        let second = "aoisd6jcsh".to_string();

        let error_string = format!(
            "Cannot perform multiplication with '{first}' and '{second}'",
            first = first,
            second = second
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TryMul(first, second));
    }

    #[test]
    fn try_div() {
        let first = "ajsldh4oi6".to_string();
        let second = "alkxc;a7i4".to_string();

        let error_string = format!(
            "Cannot perform division with '{first}' and '{second}'",
            first = first,
            second = second
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TryDiv(first, second));
    }

    #[test]
    fn try_pow() {
        let first = "als;id74cslk".to_string();
        let second = "alskd74".to_string();

        let error_string = format!(
            "Cannot raise the value '{first}' with '{second}'",
            first = first,
            second = second
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TryPow(first, second));
    }

    #[test]
    fn try_neg() {
        let first = "aslkcu4i".to_string();

        let error_string = format!("Cannot negate the value '{first}'", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TryNeg(first));
    }

    #[test]
    fn try_from() {
        let first = "askdjl4iso7".to_string();
        let second = "aiowse47scl".to_string();

        let error_string = format!(
            "Cannot convert from '{first}' to '{second}'",
            first = first,
            second = second
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TryFrom(first, second));
    }

    #[test]
    fn http() {
        let first = ";oia47wed".to_string();

        let error_string = format!(
            "There was an error processing a remote HTTP request: {first}",
            first = first
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Http(first));
    }

    #[test]
    fn channel() {
        let first = "oi57cmv".to_string();

        let error_string = format!(
            "There was an error processing a value in parallel: {first}",
            first = first
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Channel(first));
    }

    #[test]
    fn io() {
        let first = "lkas;du4".to_string();

        let error_string = format!("I/O error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Io(first));
    }

    #[test]
    fn encode() {
        let first = "5io7d".to_string();

        let error_string = format!("Key encoding error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Encode(first));
    }

    #[test]
    fn decode() {
        let first = "alkse7u47".to_string();

        let error_string = format!("Key decoding error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Decode(first));
    }

    #[test]
    fn revision() {
        let first = "zlkse7u47".to_string();

        let error_string = format!("Versioned error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Revision(first));
    }

    #[test]
    fn corrupted_index() {
        let error_string = "Index is corrupted";

        let error = QueryError::from_string(error_string).unwrap();

        assert_eq!(error, QueryError::CorruptedIndex);
    }

    #[test]
    fn no_index_found_for_match() {
        let value = "zlxk;cu5t".to_string();

        let error_string = format!(
            "There was no suitable full-text index supporting the expression '{value}'",
            value = value
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NoIndexFoundForMatch { value });
    }

    #[test]
    fn analyzer_error() {
        let first = ";litr7".to_string();

        let error_string = format!("A value can't be analyzed: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::AnalyzerError(first));
    }

    #[test]
    fn highlight_error() {
        let first = "0c9sayur".to_string();

        let error_string = format!("A value can't be highlighted: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::HighlightError(first));
    }

    #[test]
    fn bincode() {
        let first = "48e7d".to_string();

        let error_string = format!("Bincode error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Bincode(first));
    }

    #[test]
    fn fst_error() {
        let first = "894573wo".to_string();

        let error_string = format!("FstError error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::FstError(first));
    }

    #[test]
    fn utf_8_error() {
        let first = "89ew74".to_string();

        let error_string = format!("Utf8 error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Utf8Error(first));
    }

    #[test]
    fn feature_not_yet_implemented() {
        let feature = "49786sa".to_string();

        let error_string = format!("Feature not yet implemented: {feature}", feature = feature);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::FeatureNotYetImplemented { feature });
    }

    #[test]
    fn duplicated_match_ref() {
        let mr = "*".to_string();

        let error_string = format!("Duplicated Match reference: {mr}", mr = mr);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::DuplicatedMatchRef { mr });
    }

    #[test]
    fn timestamp_overflow() {
        let first = "74d".to_string();

        let error_string = format!("Timestamp arithmetic error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::TimestampOverflow(first));
    }

    #[test]
    fn internal() {
        let first = "".to_string();

        let error_string = format!("Internal database error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Internal(first));
    }

    #[test]
    fn unimplemented() {
        let first = "587edj".to_string();

        let error_string = format!("Unimplemented functionality: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Unimplemented(first));
    }

    #[test]
    fn corrupted_versionstamp_in_key() {
        let first = "yhxzudro".to_string();

        let error_string = format!("Versionstamp in key is corrupted: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::CorruptedVersionstampInKey(first));
    }

    #[test]
    fn invalid_level() {
        let first = "s4x7i".to_string();

        let error_string = format!("Invalid level '{first}'", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::InvalidLevel(first));
    }

    #[test]
    fn iam_error() {
        let first = "l8x4".to_string();

        let error_string = format!("IAM error: {first}", first = first);

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::IamError(first));
    }

    #[test]
    fn scripting_not_allowed() {
        let error_string = "Scripting functions are not allowed";

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::ScriptingNotAllowed);
    }

    #[test]
    fn function_not_allowed() {
        let first = "s4i7o".to_string();

        let error_string = format!(
            "Function '{first}' is not allowed to be executed",
            first = first
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::FunctionNotAllowed(first));
    }

    #[test]
    fn net_target_not_allowed() {
        let first = "8oe47".to_string();

        let error_string = format!(
            "Access to network target '{first}' is not allowed",
            first = first
        );

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::NetTargetNotAllowed(first));
    }

    #[test]
    fn deprecated() {
        let error_string = ""; // This can match anything

        let error = QueryError::from_string(&error_string).unwrap();

        assert_eq!(error, QueryError::Deprecated("".to_string()));
    }
}
