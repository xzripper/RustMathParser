// Parser operators.

use std::collections::HashMap;

use crate::math_parser::pconsts;

/// Math operators.
pub struct Operators {
    pub plus: &'static str,
    pub minus: &'static str,
    pub multiply: &'static str,
    pub divide: &'static str
}

impl Operators {
    /// Is operator valid.
    pub fn valid_op(&self, op: &str) -> bool {
        let pars_ops: Vec<&str> = vec![
            self.plus,
            self.minus,
            self.multiply,
            self.divide
        ];

        for oper in pars_ops {
            if op == oper {
                return true;
            }
        }

        return false;
    }

    // Represent operator.
    pub fn repr(&self, op: &str) -> &str {
        let mut reprs_of_ops: HashMap<&str, &str> = HashMap::new();

        reprs_of_ops.insert("+", "Plus");
        reprs_of_ops.insert("-", "Minus");
        reprs_of_ops.insert("*", "Multiply");
        reprs_of_ops.insert("/", "Divide");

        let pars_ops: Vec<&str> = vec![
            self.plus,
            self.minus,
            self.multiply,
            self.divide
        ];

        if !pars_ops.contains(&op) {
            return pconsts::Unknown;
        }

        return reprs_of_ops[op];
    }
}
