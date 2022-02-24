## Documentation for RustMathParser

## Parser.rs

#### Parser struct.
```rust
pub struct Parser {
    pub string: &'static str
}
```

#### Init parser from file. (Static).
```rust
pub fn from_file(filename: &str) -> Parser
```

#### Get given string.
```rust
pub fn get_str(&self) -> &str
```

#### Update string.
```rust
pub fn upd_str(&mut self, new_str: &'static str) -> ()
```

#### Get math problems.
```rust
pub fn get_problems(&self) -> Vec<&str>
```

#### Trim vector of math problems.
```rust
pub fn trim_problems<'a>(&self, problems: Vec<&'a str>) -> Vec<&'a str>
```

#### Get alredy trimmed math problems.
```rust
pub fn get_trimmed_problems(&self) -> Vec<&str>
```

#### Solve problem function.
```rust
pub fn solve_problem(&self, x: i64, y: i64, operation: &str) -> i64
```

#### Get string as integer.
```rust
pub fn as_integer(&self, entity: &str) -> i64
```

#### Get number as string.
```rust
pub fn as_string(&self, entity: i64) -> &str
```

#### Parse entities from string.
```rust
pub fn parse_entities<'a>(&'a self, string: &'a str) -> HashMap<&'a str, &'a str>
```

#### Parse all entities from math problem that separated by comma from string.
```rust
pub fn parse_all(&self) -> Vec<HashMap<&str, &str>>
```

#### Get entity from parsing results.
```rust
pub fn get_entity<'a>(&'a self, string: &'a str, entity: &str) -> &str
```

#### Format parsing result.
```rust
pub fn format_res(&self, res: Vec<HashMap<&str, &str>>) -> &str
```

#### Get parser version.
```rust
pub fn get_version(&self) -> f32
```

## PConsts.rs

#### Parser errors.
```rust
pub const ParserErrors: errs::Errors = errs::Errors {
    not_a_number: 0x8,
    invalid_operator: 0x16,
    invalid_body: 0x32,
    cant_count: 0x64
};
```

#### Parser operators.
```rust
pub const ParserOperators: operators::Operators = operators::Operators {
    plus: "+",
    minus: "-",
    multiply: "*",
    divide: "/"
};
```

#### Types.
```rust
pub const NaN: &str = "_NaN";
pub const Unknown: &str = "_Unknown";
```

#### Parser characters.
```rust
pub const parser_comma: char = ',';
pub const parser_space: char = ' ';
```

#### Parser version.
```rust
pub const version: f32 = 1.0;
```

## Operators.rs

#### Operators struct.
```rust
pub struct Operators {
    pub plus: &'static str,
    pub minus: &'static str,
    pub multiply: &'static str,
    pub divide: &'static str
}
```

#### Is operator valid.
```rust
pub fn valid_op(&self, op: &str) -> bool
```

#### Represent operator.
```rust
pub fn repr(&self, op: &str) -> &str
```

## Errs.rs

#### Errors struct.
```rust
pub struct Errors {
    pub not_a_number: i64,
    pub invalid_operator: i64,
    pub invalid_body: i64,
    pub cant_count: i64
}
```

#### Represent code to hex.
```rust
pub fn repr_code(&self, code: i64) -> &str
```

#### Return error name by code.
```rust
pub fn by_code(&self, code: i64) -> &str
```

#### Print error using user codes.
```rust
pub fn err(&self, code: i64)
```
