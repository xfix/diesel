//! Types related to describing schema, and interactions between tables.
//!
//! Most traits in this module are derived or generated by [`table!`].
//!
//! [`table!`]: ../macro.table.html
#[doc(hidden)]
pub mod joins;
mod peano_numbers;

use expression::{Expression, NonAggregate, SelectableExpression};
use query_builder::*;

pub use self::joins::JoinTo;
pub use self::peano_numbers::*;

#[cfg(feature = "with-deprecated")]
#[deprecated(since = "1.1.0", note = "Use `deserialize::Queryable` instead")]
pub use deserialize::Queryable;
#[cfg(feature = "with-deprecated")]
#[deprecated(since = "1.1.0", note = "Use `deserialize::QueryableByName` instead")]
pub use deserialize::QueryableByName;

/// Represents a type which can appear in the `FROM` clause. Apps should not
/// need to concern themselves with this trait.
///
/// Types which implement this trait include:
/// - Tables generated by the `table!` macro
/// - Internal structs which represent joins
/// - A select statement which has had no query builder methods called on it,
///   other than those which can affect the from clause.
pub trait QuerySource {
    /// The type returned by `from_clause`
    type FromClause;
    /// The type returned by `default_selection`
    type DefaultSelection: SelectableExpression<Self>;

    /// The actual `FROM` clause of this type. This is typically only called in
    /// `QueryFragment` implementations.
    fn from_clause(&self) -> Self::FromClause;
    /// The default select clause of this type, which should be used if no
    /// select clause was explicitly specified. This should always be a tuple of
    /// all the desired columns, not `star`
    fn default_selection(&self) -> Self::DefaultSelection;
}

/// A column on a database table. Types which implement this trait should have
/// been generated by the [`table!` macro](../macro.table.html).
pub trait Column: Expression {
    /// The table which this column belongs to
    type Table: Table;

    /// The name of this column
    const NAME: &'static str;
}

/// A SQL database table. Types which implement this trait should have been
/// generated by the [`table!` macro](../macro.table.html).
pub trait Table: QuerySource + AsQuery + Sized {
    /// The type returned by `primary_key`
    type PrimaryKey: SelectableExpression<Self> + NonAggregate;
    /// The type returned by `all_columns`
    type AllColumns: SelectableExpression<Self> + NonAggregate;

    /// Returns the primary key of this table.
    ///
    /// If the table has a composite primary key, this will be a tuple.
    fn primary_key(&self) -> Self::PrimaryKey;
    /// Returns a tuple of all columns belonging to this table.
    fn all_columns() -> Self::AllColumns;
}

/// Determines how many times `Self` appears in `QS`
///
/// This trait is primarily used to determine whether or not a column is
/// selectable from a given from clause. A column can be selected if its table
/// appears in the from clause *exactly once*.
///
/// We do not allow the same table to appear in a query multiple times in any
/// context where referencing that table would be ambiguous (depending on the
/// context and backend being used, this may or may not be something that would
/// otherwise result in a runtime error).
pub trait AppearsInFromClause<QS> {
    /// How many times does `Self` appear in `QS`?
    type Count;
}
