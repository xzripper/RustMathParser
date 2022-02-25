// Parser constants.

use crate::math_parser::errs;
use crate::math_parser::operators;

/// Parser errors.
pub const ParserErrors: errs::Errors = errs::Errors {
    not_a_number: 0x8,
    invalid_operator: 0x16,
    invalid_body: 0x32,
    cant_count: 0x64
};

/// Parser operators.
pub const ParserOperators: operators::Operators = operators::Operators {
    plus: "+",
    minus: "-",
    multiply: "*",
    divide: "/"
};

/// Not a Number.
pub const NaN: &str = "_NaN";

/// Unknown, using when operator is unknown.
pub const Unknown: &str = "_Unknown";

/// Comma character for parser.
pub const parser_comma: char = ',';

/// Space character for parser.
pub const parser_space: char = ' ';

/// Parser version.
pub const version: f32 = 1.1;
