use crate::solution::Solution;
use anyhow::Result;
use serde_json::Value;

fn iter_json(json: &Value, sum: &mut i64, ignore_red: bool) {
    match json {
        // Base cases
        Value::Null => return,
        Value::Bool(_) => return,
        Value::String(_) => return,

        Value::Array(arr) => {
            for val in arr {
                iter_json(val, sum, ignore_red);
            }
        }
        Value::Object(obj) => {
            let old_sum: i64 = *sum;
            let mut ignore: bool = false;

            for (_, v) in obj {
                if let Value::String(value) = v {
                    if ignore_red && value == "red" {
                        ignore = true;
                    }
                }

                iter_json(v, sum, ignore_red);
            }

            if ignore {
                *sum = old_sum;
            }
        }
        Value::Number(num) => {
            *sum += num.as_i64().unwrap();
        }
    }
}

fn count_numbers(json: &Value, ignore_red: bool) -> Result<String> {
    let mut sum: i64 = 0;

    iter_json(json, &mut sum, ignore_red);
    Ok(sum.to_string())
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    let json: Value = serde_json::from_str(&input)?;
    solution.part1 = count_numbers(&json, false)?.to_string();
    solution.part2 = count_numbers(&json, true)?.to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_red() -> Result<()> {
        let input = "[1,{\"c\":\"red\",\"b\":2},3]";
        let json: Value = serde_json::from_str(&input)?;
        assert_eq!(count_numbers(&json, true)?, "4");

        let input = "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}";
        let json: Value = serde_json::from_str(&input)?;
        assert_eq!(count_numbers(&json, true)?, "0");

        let input = "[1,\"red\",5]";
        let json: Value = serde_json::from_str(&input)?;
        assert_eq!(count_numbers(&json, true)?, "6");

        Ok(())
    }
}
