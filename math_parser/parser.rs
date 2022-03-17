// Main file.

use std::collections::HashMap;
use std::fs::read_to_string;

use crate::math_parser::pconsts;

/// Math problems parser.
pub struct Parser {
    pub string: &'static str
}

impl Parser {
    /// Init parser with string from file.
    pub fn from_file(filename: &str) -> Parser {
        let content: String = read_to_string(filename).expect("Failed to read file.");

        return Parser {
            string: Box::leak(content.into_boxed_str())
        };
    }

    /// Get given string.
    pub fn get_str(&self) -> &str {
        return self.string;
    }

    /// Update string.
    pub fn upd_str(&mut self, new_str: &'static str) -> () {
        self.string = new_str;
    }

    /// Get math problems separated by commas.
    pub fn get_problems(&self) -> Vec<&str> {
        return self.string.split(pconsts::parser_comma).collect();
    }

    /// Trim math problems vector.
    pub fn trim_problems<'a>(&self, problems: Vec<&'a str>) -> Vec<&'a str> {
        let mut trimmed_problems: Vec<&str> = vec![];

        for problem in problems {
            trimmed_problems.push(problem.trim());
        }

        return trimmed_problems;
    }

    /// Get trimmed problems right now.
    pub fn get_trimmed_problems(&self) -> Vec<&str> {
        return self.trim_problems(self.get_problems());
    }

    /// Solve math problem.
    pub fn solve_problem(&self, x: i64, y: i64, operation: &str) -> i64 {
        if operation == pconsts::ParserOperators.plus {
            return x + y;
        } else if operation == pconsts::ParserOperators.minus {
            return x - y;
        } else if operation == pconsts::ParserOperators.multiply {
            return x * y;
        } else if operation == pconsts::ParserOperators.divide {
            return x / y;
        } else {
            return -0;
        }
    }

    /// Convert string to integer.
    pub fn as_integer(&self, entity: &str) -> i64 {
        let as_int: i64 = entity.parse::<i64>().unwrap();

        return as_int;
    }

    /// Convert string to float.
    pub fn as_float(&self, entity: &str) -> f64 {
        let as_float: f64 = entity.parse::<f64>().unwrap();

        return as_float;
    }

    /// Convert integer to string.
    pub fn as_string(&self, entity: i64) -> &str {
        return Box::leak(entity.to_string().into_boxed_str());
    }

    /// Convert float to string.
    pub fn as_float_string(&self, entity: f64) -> &str {
        return Box::leak(entity.to_string().into_boxed_str())
    }

    /// Is string matches digit template.
    pub fn is_integer(&self, entity: &str) -> bool {
        let as_str_a_int: bool = entity.parse::<i64>().is_ok();

        return as_str_a_int;
    }

    /// Is string matches float template.
    pub fn is_float(&self, entity: &str) -> bool {
        let is_str_a_float: bool = entity.parse::<f64>().is_ok() && entity.contains(pconsts::parser_dot);

        return is_str_a_float;
    }

    /// Parse entities from string.
    pub fn parse_entities<'a>(&'a self, string: &'a str) -> HashMap<&'a str, &'a str> {
        let mut parsed: HashMap<&str, &str> = HashMap::new();

        let entities: Vec<&str> = string.split(pconsts::parser_space).collect();

        if entities.len() < 3 || entities.len() > 3 {
            pconsts::ParserErrors.err(pconsts::ParserErrors.invalid_body);

            return HashMap::from([
                ("first", pconsts::NaN),
                ("second", pconsts::NaN),
                ("operator", pconsts::Unknown),
                ("result", pconsts::NaN)
            ]);
        }

        let first_entity: &str = entities[0];
        let operator: &str = entities[1];
        let second_entity: &str = entities[2];

        if self.is_integer(first_entity) || self.is_float(first_entity) {
            parsed.insert("first", first_entity);
        } else {
            pconsts::ParserErrors.err(pconsts::ParserErrors.not_a_number);

            parsed.insert("first", pconsts::NaN);
        }

        if self.is_integer(second_entity) || self.is_float(second_entity) {
            parsed.insert("second", second_entity);
        } else {
            pconsts::ParserErrors.err(pconsts::ParserErrors.not_a_number);

            parsed.insert("second", pconsts::NaN);
        }

        if pconsts::ParserOperators.valid_op(operator) {
            parsed.insert("operator", Box::leak(format!("{} ({})", pconsts::ParserOperators.repr(operator), operator).into_boxed_str()));
        } else {
            pconsts::ParserErrors.err(pconsts::ParserErrors.invalid_operator);

            parsed.insert("operator", Box::leak(format!("{} ({})", pconsts::Unknown, operator).into_boxed_str()));
        }

        if parsed["first"] != pconsts::NaN && parsed["second"] != pconsts::NaN && parsed["operator"] != Box::leak(format!("{} ({})", pconsts::Unknown, operator).into_boxed_str()) {
            if second_entity.replace(pconsts::parser_dot, "").chars().all(|character| character == '0') && operator == pconsts::ParserOperators.divide {
                pconsts::ParserErrors.err(pconsts::ParserErrors.cant_count);

                parsed.insert("result", pconsts::NaN);
            } else {
                if self.is_float(first_entity) || self.is_float(second_entity) {
                    let mut second_float: f64 = 0.0;

                    if self.is_float(second_entity) {
                        second_float = self.as_float(second_entity);
                    }

                    let mut result: f64;

                    if self.is_float(first_entity) {
                        result = self.as_float(first_entity);
                    } else {
                        if self.is_integer(first_entity) {
                            result = self.as_integer(first_entity) as f64;
                        } else {
                            pconsts::ParserErrors.err(pconsts::ParserErrors.not_a_number);

                            parsed.insert("result", pconsts::NaN);
                            parsed.insert("first", pconsts::NaN);

                            result = -0.0;
                        }
                    }

                    if operator == pconsts::ParserOperators.plus {
                        result += second_float;
                    } else if operator == pconsts::ParserOperators.minus {
                        result -= second_float;
                    } else if operator == pconsts::ParserOperators.multiply {
                        result *= second_float;
                    } else if operator == pconsts::ParserOperators.divide {
                        result /= second_float;
                    }

                    parsed.insert("result", self.as_float_string(result));
                } else {
                    parsed.insert(
                        "result", self.as_string(
                            self.solve_problem(
                                self.as_integer(first_entity),
                                self.as_integer(second_entity),
                                operator
                            )
                        )
                    );
                }
            }
        } else {
            parsed.insert("result", pconsts::NaN);
        }

        return parsed;
    }

    /// Parse all math problems separated by comma.
    pub fn parse_all(&self) -> Vec<HashMap<&str, &str>> {
        let math_problems: Vec<&str> = self.get_trimmed_problems();
        let mut parsed: Vec<HashMap<&str, &str>> = vec![];

        for math_problem in math_problems {
            parsed.push(self.parse_entities(math_problem));
        }

        return parsed;
    }

    /// Get entity from parse results.
    pub fn get_entity<'a>(&'a self, string: &'a str, entity: &str) -> &str {
        let entities: HashMap<&str, &str> = self.parse_entities(string);

        if !entities.contains_key(entity) {
            return pconsts::Unknown;
        }

        return entities[entity];
    }

    /// Format parsing result.
    pub fn format_res(&self, res: Vec<HashMap<&str, &str>>) -> &str {
        let mut formatted: String = String::new();

        for pars_res in res {
            formatted.push_str(Box::leak(format!("First: {}, Second: {}, Operator: {}, Result: {}.\n", pars_res["first"], pars_res["second"], pars_res["operator"], pars_res["result"]).into_boxed_str()));
        }

        return Box::leak(formatted.into_boxed_str());
    }

    /// Get parser version.
    pub fn get_version(&self) -> f32 {
        return pconsts::version;
    }
}
