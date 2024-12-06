use std::collections::HashSet;

type Direction = (i32, i32);

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn update(self, dir: (i32, i32)) -> Position {
        Self {
            x: self.x + dir.0,
            y: self.y + dir.1,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Default)]
struct Grid {
    obstacles: HashSet<Position>,
    start: Position,
    n: i32,
    m: i32,
}

impl Grid {
    fn advance(&self, position: &mut Position, dir: &mut Direction) -> bool {
        let next = position.update(*dir);
        if self.obstacles.contains(&next) {
            *dir = match dir {
                (1, 0) => (0, -1),  // down -> left
                (-1, 0) => (0, 1),  // up -> right
                (0, 1) => (1, 0),   // right -> down
                (0, -1) => (-1, 0), // left -> up
                _ => unreachable!(),
            };
        } else {
            *position = next;
        }
        let oob = position.x < 0 || position.x >= self.n || position.y < 0 || position.y >= self.m;
        !oob
    }
}

fn get_grid(input: &str) -> Grid {
    let mut grid = Grid::default();
    input.trim().lines().for_each(|line| {
        grid.m = line.trim().len() as i32;
        line.trim().bytes().enumerate().for_each(|(i, c)| match c {
            b'#' => {
                grid.obstacles.insert((grid.n, i as i32).into());
            }
            b'^' => {
                grid.start = (grid.n, i as i32).into();
            }
            _ => {}
        });
        grid.n += 1;
    });
    grid
}

fn get_path(grid: &Grid) -> HashSet<Position> {
    let mut position = grid.start;
    let mut dir = (-1, 0);
    let mut path = HashSet::new();
    path.insert(position);
    while grid.advance(&mut position, &mut dir) {
        path.insert(position);
    }
    path
}

pub fn p1(input: &str) -> u32 {
    let grid = get_grid(input);
    let path = get_path(&grid);
    path.len() as u32
}

pub fn p2(input: &str) -> u32 {
    let mut grid = get_grid(input);
    let path = get_path(&grid);

    path.into_iter()
        .filter(|new_obstacle| {
            assert!(!grid.obstacles.contains(&new_obstacle));
            grid.obstacles.insert(*new_obstacle);
            let mut position = grid.start;
            let mut dir = (-1, 0);
            let mut seen = HashSet::new();
            seen.insert((position, dir));
            while grid.advance(&mut position, &mut dir) {
                if !seen.insert((position, dir)) {
                    grid.obstacles.remove(&new_obstacle);
                    return true;
                }
            }
            grid.obstacles.remove(&new_obstacle);
            return false;
        })
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 41);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 6);
    }
}
