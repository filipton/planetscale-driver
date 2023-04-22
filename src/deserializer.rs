use crate::{structs::ExecuteResponse, utils::from_base64, Parser};
use anyhow::{Context, Result};

pub trait Deserializer {
    fn deserialize_raw(input: Vec<&str>) -> Result<Self>
    where
        Self: Sized;
}

impl ExecuteResponse {
    pub fn deserialize<T>(&self) -> Result<T>
    where
        T: Deserializer,
    {
        if let Some(res) = &self.result {
            if let Some(rows) = &res.rows {
                if rows.len() != 1 {
                    anyhow::bail!("Expected 1 row, got {}", rows.len());
                }

                let row = &rows[0];
                let row_str = from_base64(&row.values);
                let row_str = String::from_utf8(row_str).unwrap();

                let lengths: Vec<usize> = row
                    .lengths
                    .iter()
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect();

                let mut row_vec: Vec<&str> = Vec::new();
                let mut last = 0;
                for length in lengths {
                    row_vec.push(&row_str[last..(last + length)]);
                    last += length;
                }

                let res = T::deserialize_raw(row_vec).context("Failed to deserialize row")?;
                return Ok(res);
            }
        }

        anyhow::bail!("No results found");
    }

    pub fn deserialize_multiple<T>(&self) -> Result<Vec<T>>
    where
        T: Deserializer,
    {
        if let Some(res) = &self.result {
            if let Some(rows) = &res.rows {
                let mut out: Vec<T> = Vec::new();
                for row in rows {
                    let row_str = from_base64(&row.values);
                    let row_str = String::from_utf8(row_str).unwrap();

                    let lengths: Vec<usize> = row
                        .lengths
                        .iter()
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect();

                    let mut row_vec: Vec<&str> = Vec::new();
                    let mut last = 0;
                    for length in lengths {
                        row_vec.push(&row_str[last..(last + length)]);
                        last += length;
                    }

                    out.push(T::deserialize_raw(row_vec).context("Failed to deserialize row")?);
                }

                return Ok(out);
            }
        }

        anyhow::bail!("No results found");
    }

    pub fn deserialize_scalar<T>(&self) -> Result<T>
    where
        T: Parser,
    {
        if let Some(res) = &self.result {
            if let Some(rows) = &res.rows {
                if rows.len() != 1 || rows[0].lengths.len() != 1 {
                    anyhow::bail!(
                        "Expected 1 row 1 value, got {} rows, {} values",
                        rows.len(),
                        rows[0].lengths.len()
                    );
                }

                let row = &rows[0];
                let row_str = from_base64(&row.values);
                let row_str = String::from_utf8(row_str).unwrap();

                let res = T::custom_parse(&row_str)
                    .ok()
                    .context(format!("Failed to deserialize scalar {:?}", row_str))?;
                return Ok(res);
            }
        }

        anyhow::bail!("No results found");
    }
}
