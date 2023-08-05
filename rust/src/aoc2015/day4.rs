use crate::solution::Solution;
use anyhow::Result;
use md5::{Digest, Md5};

fn find_postfix(prefix: &str, num_zeros: u8) -> Result<u32> {
    for i in 1..u32::MAX {
        let mut key = prefix.to_owned();
        key.push_str(&i.to_string());

        let result = Md5::digest(key.clone());
        let hash_arr: [u8; 16] = result.into();
        let hash = u128::from_be_bytes(hash_arr);

        let leading = hash >> (128 - (4 * num_zeros));
        if leading == 0 {
            return Ok(i);
        }
    }

    Ok(0)
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = find_postfix(&input, 5)?.to_string();
    solution.part2 = find_postfix(&input, 6)?.to_string();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_postfix() -> Result<()> {
        assert_eq!(find_postfix("abcdef", 5)?, 609043);
        assert_eq!(find_postfix("pqrstuv", 5)?, 1048970);

        Ok(())
    }
}
