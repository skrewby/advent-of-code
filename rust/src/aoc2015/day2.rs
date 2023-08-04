use crate::solution::Solution;
use anyhow::Result;

fn get_dimensions(dim_str: &str) -> Vec<u32> {
    dim_str
        .split('x')
        .map(|x| x.to_string().parse::<u32>().unwrap())
        .collect()
}

fn get_face_areas(dim: &[u32]) -> Vec<u32> {
    vec![dim[0] * dim[1], dim[0] * dim[2], dim[1] * dim[2]]
}

fn calculate_surface_area(dimensions: &str) -> u32 {
    let dim = get_dimensions(dimensions);
    let sides = get_face_areas(&dim);
    let smallest_side = sides.iter().min().unwrap();
    let area: u32 = sides.iter().map(|x| x * 2).sum();

    smallest_side + area
}

fn calculate_ribbon_required(dimensions: &str) -> u32 {
    let mut dim: Vec<u32> = get_dimensions(dimensions);
    dim.sort_unstable();

    let mut total = dim[0] * 2 + dim[1] * 2;
    total += dim[0] * dim[1] * dim[2];

    total
}

fn day_2(input: &str) -> String {
    let mut total_ribbon = 0;
    for dim in input.split('\n') {
        if !dim.is_empty() {
            total_ribbon += calculate_ribbon_required(dim);
        }
    }

    total_ribbon.to_string()
}

fn day_1(input: &str) -> String {
    let mut total_area = 0;
    for dim in input.split('\n') {
        if !dim.is_empty() {
            total_area += calculate_surface_area(dim);
        }
    }

    total_area.to_string()
}

pub fn solve(input: String, solution: &mut Solution) -> Result<()> {
    solution.part1 = day_1(&input);
    solution.part2 = day_2(&input);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_surface_area() {
        assert_eq!(calculate_surface_area("2x3x4"), 58);
        assert_eq!(calculate_surface_area("1x1x10"), 43);
    }

    #[test]
    fn test_get_dimensions() {
        assert_eq!(get_dimensions("1x2x3"), vec![1, 2, 3]);
    }

    #[test]
    fn test_get_face_areas() {
        assert_eq!(get_face_areas(&[2, 3, 4]), vec![6, 8, 12]);
    }

    #[test]
    fn test_calculate_ribbon_required() {
        assert_eq!(calculate_ribbon_required("2x3x4"), 34);
        assert_eq!(calculate_ribbon_required("1x1x10"), 14);
    }
}
