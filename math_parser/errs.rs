// Parser errors.

use std::collections::HashMap;

/// Language error codes.
pub struct Errors {
    pub not_a_number: i64,
    pub invalid_operator: i64,
    pub invalid_body: i64,
    pub cant_count: i64
}

impl Errors {
    /// Represent error code.
    pub fn repr_code(&self, code: i64) -> &str {
        return Box::leak(format!("0x{:x}", code).into_boxed_str());
    }

    /// Get name by code.
    pub fn by_code(&self, code: i64) -> &str {
        let mut reprs_by_code: HashMap<i64, &str> = HashMap::new();

        reprs_by_code.insert(self.not_a_number, "not_a_number");
        reprs_by_code.insert(self.invalid_operator, "invalid_operator");
        reprs_by_code.insert(self.invalid_body, "invalid_body");
        reprs_by_code.insert(self.cant_count, "cant_count");

        return reprs_by_code[&code];
    }

    /// Print error.
    pub fn err(&self, code: i64) -> () {
        println!("{}", format!("error: {} [{}] ({});", self.repr_code(code), code, self.by_code(code)));
    }
}
