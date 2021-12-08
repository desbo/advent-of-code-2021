use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(",").collect();
        match v.as_slice() {
            [x, y] => Ok(Point::new(
                u16::from_str(x.trim()).unwrap(),
                u16::from_str(y.trim()).unwrap(),
            )),
            _ => Err(()),
        }
    }
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Line(Point, Point);

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("->").collect();

        match v.as_slice() {
            [l, r] => Ok(Line(Point::from_str(l)?, Point::from_str(r)?)),
            _ => Err(()),
        }
    }
}

impl Line {
    fn horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn vertical(&self) -> bool {
        self.0.x == self.1.x
    }
}

struct Plane(Vec<Vec<u16>>);

impl Display for Plane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|v| match v {
                            0 => ".".to_string(),
                            x => x.to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Plane {
    fn new(width: usize, height: usize) -> Self {
        Plane(vec![vec![0; width]; height])
    }

    fn add_line(&mut self, line: &Line) {
        let min_x = min(line.0.x, line.1.x) as usize;
        let max_x = max(line.0.x, line.1.x) as usize;
        let min_y = min(line.0.y, line.1.y) as usize;
        let max_y = max(line.0.y, line.1.y) as usize;

        if line.horizontal() || line.vertical() {
            for y in min_y..max_y + 1 {
                for x in min_x..max_x + 1 {
                    self.0[y][x] += 1
                }
            }
        } else {
            let leftmost_point = if min_x == line.0.x as usize {
                &line.0
            } else {
                &line.1
            };

            let upwards = leftmost_point.y < max_y as u16;

            let mut x = leftmost_point.x as usize;
            let mut y = leftmost_point.y as usize;

            while x <= max_x {
                self.0[y][x] += 1;
                x += 1;

                if upwards {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    fn add_lines<'a>(&mut self, lines: impl Iterator<Item = &'a Line>) {
        lines.for_each(|l| self.add_line(&l))
    }

    fn for_each_point(&self, mut f: impl FnMut(Point, u16)) {
        self.0.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, value)| {
                f(
                    Point {
                        x: x as u16,
                        y: y as u16,
                    },
                    *value,
                )
            })
        })
    }

    fn count_if(&self, mut p: impl FnMut(Point, u16) -> bool) -> usize {
        let mut result = 0;

        self.for_each_point(|point, value| {
            if p(point, value) {
                result += 1;
            }
        });

        result
    }
}

pub fn part1(lines: &[Line]) -> u16 {
    let mut plane = Plane::new(1000, 1000);

    plane.add_lines(lines.iter().filter(|l| l.horizontal() || l.vertical()));
    plane.count_if(|_p, v| v > 1) as u16
}

pub fn part2(lines: &[Line]) -> u16 {
    let mut plane = Plane::new(1000, 1000);

    plane.add_lines(lines.iter());
    plane.count_if(|_p, v| v > 1) as u16
}
