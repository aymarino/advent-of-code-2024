use itertools::iproduct;

fn get_offset(map: &[Vec<u8>], dx: i32, dy: i32, x: usize, y: usize) -> Option<u8> {
    let x = x as i32 + dx;
    let y = y as i32 + dy;
    if x < 0 || x as usize >= map.len() {
        return None;
    }
    if y < 0 || y as usize >= map[x as usize].len() {
        return None;
    }
    Some(map[x as usize][y as usize])
}

fn get_map(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect()
}

pub fn p1(input: &str) -> u32 {
    let map = get_map(input);
    iproduct!(0..map.len(), 0..map[0].len())
        .map(|(i, j)| {
            if map[i][j] != b'X' {
                return 0;
            }
            [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ]
            .into_iter()
            .filter(|(dx, dy)| {
                let expected = b"XMAS";
                (1..=3).all(|step| {
                    get_offset(&map, dx * step, dy * step, i, j) == Some(expected[step as usize])
                })
            })
            .count()
        })
        .sum::<usize>() as u32
}

pub fn p2(input: &str) -> u32 {
    let map = get_map(input);
    iproduct!(0..map.len(), 0..map[0].len())
        .filter(|(i, j)| {
            if map[*i][*j] != b'A' {
                return false;
            }

            let get = |dx, dy| get_offset(&map, dx, dy, *i, *j);

            let left_diag_sam = get(-1, -1) == Some(b'S') && get(1, 1) == Some(b'M');
            let left_diag_mas = get(-1, -1) == Some(b'M') && get(1, 1) == Some(b'S');
            let left_diag = left_diag_sam || left_diag_mas;

            let right_diag_sam = get(-1, 1) == Some(b'S') && get(1, -1) == Some(b'M');
            let right_diag_mas = get(-1, 1) == Some(b'M') && get(1, -1) == Some(b'S');
            let right_diag = right_diag_sam || right_diag_mas;

            left_diag && right_diag
        })
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 18);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 9);
    }
}
