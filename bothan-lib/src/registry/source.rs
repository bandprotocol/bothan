// ! Source definitions and routing operations for signal computation.
//!
//! This module provides structures for defining data sources and routing operations
//! that transform source data before it is processed. It enables the flexible
//! combination and transformation of data from various sources.
//!
//! The module provides:
//!
//! - The [`Operation`] enum which defines basic arithmetic operations
//! - The [`OperationRoute`] struct which pairs operations with dependent signals
//! - The [`SourceQuery`] struct which specifies where to obtain input data
//!
//! # Source Routing
//!
//! Sources can be configured with routes that apply transformations using values
//! from other signals. This enables complex relationships between signals, such as:
//!
//! - Converting between different units (e.g., USDT to USD)
//! - Adjusting values using scaling factors
//! - Applying corrections based on other market data
//!
//! Routing operations are applied sequentially, with each operation using the
//! result of the previous operation as its first operand.

use bincode::{Decode, Encode};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use serde::{Deserialize, Serialize};

/// Arithmetic operations that can be applied in routing transformations.
///
/// The `Operation` enum represents the four basic arithmetic operations that
/// can be used to transform values during the routing process. Each operation
/// is designed to safely handle potential numerical errors using checked operations.
///
/// # Variants
///
/// * `Add` - Addition operation (+)
/// * `Subtract` - Subtraction operation (-)
/// * `Multiply` - Multiplication operation (*)
/// * `Divide` - Division operation (/)
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::source::Operation;
/// use rust_decimal::Decimal;
///
/// // Create a multiplication operation
/// let operation = Operation::Multiply;
///
/// // Apply the operation to two values
/// let a = Decimal::new(10, 0);  // 10.0
/// let b = Decimal::new(15, 0);  // 15.0
/// let result = operation.execute(a, b).unwrap();
///
/// assert_eq!(result, Decimal::new(150, 0));  // 150.0
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum Operation {
    /// Addition operation (+)
    #[serde(rename = "+")]
    Add,
    /// Subtraction operation (-)
    #[serde(rename = "-")]
    Subtract,
    /// Multiplication operation (*)
    #[serde(rename = "*")]
    Multiply,
    /// Division operation (/)
    #[serde(rename = "/")]
    Divide,
}

impl Operation {
    /// Executes the arithmetic operation on two values.
    ///
    /// This method applies the operation represented by the enum variant to the
    /// provided operands. It uses checked operations to prevent numerical errors
    /// such as overflow or division by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::registry::source::Operation;
    ///
    /// let add_op = Operation::Add;
    /// assert_eq!(add_op.execute(5, 3), Some(8));
    ///
    /// let div_op = Operation::Divide;
    /// assert_eq!(div_op.execute(10, 2), Some(5));
    /// assert_eq!(div_op.execute(10, 0), None);  // Division by zero returns None
    /// ```
    pub fn execute<T>(&self, a: T, b: T) -> Option<T>
    where
        T: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + Copy,
    {
        match self {
            Operation::Add => a.checked_add(&b),
            Operation::Subtract => a.checked_sub(&b),
            Operation::Multiply => a.checked_mul(&b),
            Operation::Divide => a.checked_div(&b),
        }
    }
}

/// A transformation that applies an operation using a value from another signal.
///
/// The `OperationRoute` struct defines a step in a routing transformation chain.
/// It specifies a signal whose value should be used as the second operand in
/// an operation, with the first operand being either the original source value
/// or the result of previous routing operations.
///
/// Routes are applied sequentially, allowing for complex transformations to be
/// built from simple arithmetic operations.
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::source::{OperationRoute, Operation};
///
/// // Create a route that multiplies by the "USDT-USD" signal's value
/// let route = OperationRoute::new("USDT-USD", Operation::Multiply);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct OperationRoute {
    /// The identifier of the signal whose value will be used in the operation.
    ///
    /// When this route is applied, the system will look up the current value
    /// of the signal with this ID and use it as the second operand in the operation.
    pub signal_id: String,

    /// The arithmetic operation to apply.
    ///
    /// This defines how the current value will be combined with the value
    /// from the referenced signal.
    pub operation: Operation,
}

impl OperationRoute {
    /// Creates a new operation route.
    ///
    /// This constructor creates an operation route that will apply the specified
    /// operation using the value of the referenced signal.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::registry::source::{OperationRoute, Operation};
    ///
    /// // Create a route that divides by the "USD-EUR" signal's value
    /// let route = OperationRoute::new("USD-EUR", Operation::Divide);
    /// ```
    pub fn new<T: Into<String>>(signal_id: T, operation: Operation) -> Self {
        OperationRoute {
            signal_id: signal_id.into(),
            operation,
        }
    }
}

/// A specification for retrieving and transforming data from a source.
///
/// The `SourceQuery` struct defines where to obtain data and how to transform it
/// before processing. It specifies a data source, an identifier to query within
/// that source, and optionally a series of routing operations to apply to the
/// retrieved value.
///
/// This structure is a key component of signal definitions, allowing each signal
/// to combine and transform data from multiple sources.
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::source::{SourceQuery, OperationRoute, Operation};
///
/// // Create a query for BTC/USDT from Binance, converted to USD
/// let query = SourceQuery::new(
///     "binance",
///     "btcusdt",
///     vec![
///         // Convert from USDT to USD using the USDT-USD signal
///         OperationRoute::new("USDT-USD", Operation::Multiply),
///     ]
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct SourceQuery {
    /// The identifier of the data source.
    ///
    /// This corresponds to a registered worker that can provide data for this query.
    /// Examples might include exchange names like "binance" or "coinbase".
    pub source_id: String,

    /// The query identifier to use within the data source.
    ///
    /// This specifies what data to retrieve from the source, such as a trading pair
    /// like "btcusdt" or an asset identifier like "bitcoin".
    #[serde(rename = "id")]
    pub query_id: String,

    /// Routing operations to apply to the retrieved value.
    ///
    /// These operations are applied sequentially to transform the source value
    /// before it is sent to the processor. If not provided, no transformations
    /// will be applied.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<OperationRoute>,
}

impl SourceQuery {
    /// Creates a new source query.
    ///
    /// This constructor creates a source query that will retrieve data from the
    /// specified source and apply the provided routing operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::registry::source::{SourceQuery, OperationRoute, Operation};
    ///
    /// // Create a query for ETH/USD from Coinbase with no transformations
    /// let direct_query = SourceQuery::new("coinbase", "ETH-USD", vec![]);
    ///
    /// // Create a query for ETH/BTC from Kraken, converted to USD
    /// let converted_query = SourceQuery::new(
    ///     "kraken",
    ///     "ethbtc",
    ///     vec![
    ///         // Convert from BTC to USD using the BTC-USD signal
    ///         OperationRoute::new("BTC-USD", Operation::Multiply),
    ///     ]
    /// );
    /// ```
    pub fn new<T, U>(source_id: T, query_id: U, routes: Vec<OperationRoute>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        SourceQuery {
            source_id: source_id.into(),
            query_id: query_id.into(),
            routes,
        }
    }
}
