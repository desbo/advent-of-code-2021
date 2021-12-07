use std::array::IntoIter;
use std::cmp::{max, min};

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

struct Line(Point, Point);

impl Line {
    fn horizontal(&self) -> bool {
        self.0.x == self.1.x
    }

    fn vertical(&self) -> bool {
        self.0.y == self.1.y
    }
}

struct Plane(Vec<Vec<u16>>);

impl Plane {
    fn new(width: usize, height: usize) -> Self {
        Plane(vec![vec![0; width]; height])
    }

    fn add_line(&mut self, line: &Line) {
        if line.horizontal() {
            let min_y = min(line.0.y, line.1.y) as usize;
            let max_y = max(line.0.y, line.1.y) as usize;

            for y in min_y..max_y {
                self.0[line.0.x as usize][y] += 1
            }
        }

        if line.vertical() {
            let min_x = min(line.0.x, line.1.x) as usize;
            let max_x = max(line.0.x, line.1.x) as usize;

            for x in min_x..max_x {
                self.0[x][line.0.y as usize] += 1
            }
        }
    }

    fn plot(&mut self, lines: &[Line]) {
        lines.iter().for_each(|l| self.add_line(l))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d() {
        let mut p = Plane::new(10, 10);
        assert_eq!(p.count_if(|_p, value| value == 0), 100);

        let l = Line(Point::new(0, 0), Point::new(0, 5));
        p.add_line(&l);
        assert_eq!(p.count_if(|_p, value| value == 1), 5);
    }
}
