use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Default, Debug)]
struct Grid {
    antennas: HashMap<u8, Vec<(i32, i32)>>,
    n: i32,
    m: i32,
}

fn get_grid(input: &str) -> Grid {
    let mut grid = Grid::default();
    input.trim().lines().for_each(|line| {
        grid.m = line.trim().len() as i32;
        line.trim().bytes().enumerate().for_each(|(i, c)| match c {
            b'.' => {}
            _ => {
                assert!(c.is_ascii_alphanumeric());
                grid.antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((grid.n, i as i32))
            }
        });
        grid.n += 1;
    });
    grid
}

fn add_antinode(grid: &Grid, antinodes: &mut HashSet<(i32, i32)>, loc: (i32, i32)) -> bool {
    if loc.0 < 0 || loc.0 >= grid.n {
        return false;
    }
    if loc.1 < 0 || loc.1 >= grid.m {
        return false;
    }
    antinodes.insert(loc);
    return true;
}

pub fn p1(input: &str) -> u64 {
    let grid = get_grid(input);
    let mut antinodes = HashSet::new();
    grid.antennas.iter().for_each(|(_, locs)| {
        locs.iter().tuple_combinations().for_each(|(p1, p2)| {
            let dx = p1.0 - p2.0;
            let dy = p1.1 - p2.1;
            add_antinode(&grid, &mut antinodes, (p1.0 + dx, p1.1 + dy));
            add_antinode(&grid, &mut antinodes, (p2.0 - dx, p2.1 - dy));
        });
    });
    antinodes.len() as u64
}

pub fn p2(input: &str) -> u64 {
    let grid = get_grid(input);
    let mut antinodes = HashSet::new();
    grid.antennas.iter().for_each(|(_, locs)| {
        locs.iter().tuple_combinations().for_each(|(p1, p2)| {
            let dx = p1.0 - p2.0;
            let dy = p1.1 - p2.1;
            let mut factor = 0;
            while add_antinode(
                &grid,
                &mut antinodes,
                (p1.0 + factor * dx, p1.1 + factor * dy),
            ) {
                factor += 1;
            }
            factor = 0;
            while add_antinode(
                &grid,
                &mut antinodes,
                (p2.0 - factor * dx, p2.1 - factor * dy),
            ) {
                factor += 1;
            }
        });
    });
    antinodes.len() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 14);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 34);
    }
}
