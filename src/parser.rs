use anyhow::Result;

pub trait Parser {
    fn custom_parse(input: &str) -> Result<Self>
    where
        Self: Sized;
}

impl Parser for String {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.to_string())
    }
}

impl Parser for isize {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for i128 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for i64 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for i32 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for i16 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for i8 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for usize {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for u128 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for u64 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for u32 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for u16 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for u8 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for f64 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for f32 {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}

impl Parser for bool {
    fn custom_parse(input: &str) -> Result<Self> {
        return match input {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(anyhow::anyhow!("Invalid boolean")),
        };
    }
}

impl Parser for char {
    fn custom_parse(input: &str) -> Result<Self> {
        Ok(input.parse()?)
    }
}
