use crate::{structs::ExecuteResponse, utils::from_base64, Deserializer};
use anyhow::{Context, Result};

impl ExecuteResponse {
    pub fn deserialize<T>(&self) -> Result<T>
    where
        T: Deserializer,
    {
        if let Some(res) = &self.result {
            if let Some(_fields) = &res.fields {
                /*
                tmp.types = fields
                    .iter()
                    .map(|f| ParsedFieldType::from_str(&f.type_field))
                    .collect();
                */

                if res.rows.len() != 1 {
                    anyhow::bail!("Expected 1 row, got {}", res.rows.len());
                }

                let row = &res.rows[0];
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
            if let Some(_fields) = &res.fields {
                /*
                tmp.types = fields
                    .iter()
                    .map(|f| ParsedFieldType::from_str(&f.type_field))
                    .collect();
                */

                let mut out: Vec<T> = Vec::new();
                for row in &res.rows {
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
}
