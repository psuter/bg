use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    o_points: [u8; 24],
    x_points: [u8; 24],
    o_bar: u8,
    x_bar: u8,
    o_home: u8,
    x_home: u8,
}

impl Position {
    pub fn point_o_value(&self, point: u8) -> u8 {
        self.o_points[usize::from(point)]
    }

    pub fn point_x_value(&self, point: u8) -> u8 {
        self.x_points[usize::from(point)]
    }

    pub fn o_bar_value(&self) -> u8 {
        self.o_bar
    }

    pub fn x_bar_value(&self) -> u8 {
        self.x_bar
    }

    pub fn o_can_bear_off(&self) -> bool {
        self.o_home
            + self.point_o_value(0)
            + self.point_o_value(1)
            + self.point_o_value(2)
            + self.point_o_value(3)
            + self.point_o_value(4)
            + self.point_o_value(5)
            == 15
    }

    pub fn o_has_won(&self) -> bool {
        self.o_home == 15
    }

    pub fn x_has_won(&self) -> bool {
        self.x_home == 15
    }

    pub fn is_over(&self) -> bool {
        self.o_has_won() || self.x_has_won()
    }

    pub fn o_has_gammoned(&self) -> bool {
        self.o_has_won() && self.x_home == 0
    }

    pub fn x_has_gammoned(&self) -> bool {
        self.x_has_won() && self.o_home == 0
    }

    pub fn o_has_backgammoned(&self) -> bool {
        self.o_has_gammoned()
            && (self.x_bar
                + self.point_x_value(0)
                + self.point_x_value(1)
                + self.point_x_value(2)
                + self.point_x_value(3)
                + self.point_x_value(4)
                + self.point_x_value(5))
                > 0
    }

    pub fn x_has_backgammoned(&self) -> bool {
        self.x_has_gammoned()
            && (self.o_bar
                + self.point_o_value(23)
                + self.point_o_value(22)
                + self.point_o_value(21)
                + self.point_o_value(20)
                + self.point_o_value(19)
                + self.point_o_value(18))
                > 0
    }

    pub fn initial() -> Position {
        Position::make(
            &[(6, 5), (8, 3), (13, 5), (24, 2)],
            &[(1, 2), (12, 5), (17, 3), (19, 5)],
            0,
            0,
            0,
            0,
        )
    }

    pub fn make(
        o_points: &[(u8, u8)],
        x_points: &[(u8, u8)],
        o_bar: u8,
        x_bar: u8,
        o_home: u8,
        x_home: u8,
    ) -> Position {
        let mut o_pts = [0; 24];
        let mut x_pts = [0; 24];

        for (pt, val) in o_points.iter() {
            if *pt < 1 || *pt > 24 || *val < 1 || *val > 15 {
                panic!("invalid position");
            }
            o_pts[usize::from(*pt) - 1] = *val;
        }

        for (pt, val) in x_points.iter() {
            if *pt < 1 || *pt > 24 || *val < 1 || *val > 15 {
                panic!("invalid position");
            }
            x_pts[usize::from(*pt) - 1] = *val;
        }

        let result = Position {
            o_points: o_pts,
            x_points: x_pts,
            o_bar,
            x_bar,
            o_home,
            x_home,
        };

        if !result.is_valid() {
            panic!("Invalid position.");
        }

        result
    }

    /// Returns a new position with one checker moved from point `from` to point `to`.
    pub fn with_o_move(&self, from: u8, to: u8) -> Position {
        assert!(to < from);

        if self.point_o_value(from) < 1 {
            panic!("Cannot move non-existing checker.");
        }

        if self.point_x_value(to) > 1 {
            panic!("Cannot move to made point.");
        }

        let hits = self.point_x_value(to) == 1;
        let from = usize::from(from);
        let to = usize::from(to);

        let mut new_o_points = self.o_points;
        new_o_points[to] = self.o_points[to] + 1;
        new_o_points[from] = self.o_points[from] - 1;

        let new_x_points = if hits {
            let mut new_x_points = self.x_points;
            new_x_points[to] = 0;
            new_x_points
        } else {
            self.x_points
        };

        Position {
            o_points: new_o_points,
            x_points: new_x_points,
            o_bar: self.o_bar,
            x_bar: if hits { self.x_bar + 1 } else { self.x_bar },
            o_home: self.o_home,
            x_home: self.x_home,
        }
    }

    /// Returns a new position with one checker moved from the bar to point `to`.
    pub fn with_o_entering(&self, to: u8) -> Position {
        if self.o_bar_value() < 1 {
            panic!("Cannot enter non-existing checker.");
        }

        if self.point_x_value(to) > 1 {
            panic!("Cannot move to made point.");
        }

        let hits = self.point_x_value(to) == 1;
        let to = usize::from(to);

        let mut new_o_points = self.o_points;
        new_o_points[to] = self.o_points[to] + 1;

        let new_x_points = if hits {
            let mut new_x_points = self.x_points;
            new_x_points[to] = 0;
            new_x_points
        } else {
            self.x_points
        };

        Position {
            o_points: new_o_points,
            x_points: new_x_points,
            o_bar: self.o_bar - 1,
            x_bar: if hits { self.x_bar + 1 } else { self.x_bar },
            o_home: self.o_home,
            x_home: self.x_home,
        }
    }

    /// Returns a new position with one checker off.
    pub fn with_o_bearing_off(&self, from: u8) -> Position {
        if !self.o_can_bear_off() {
            panic!("Cannot bear-off.");
        }

        if self.point_o_value(from) < 1 {
            panic!("Cannot move non-existing checker.");
        }

        let from = usize::from(from);

        let mut new_o_points = self.o_points;
        new_o_points[from] = self.o_points[from] - 1;

        Position {
            o_points: new_o_points,
            x_points: self.x_points,
            o_bar: self.o_bar,
            x_bar: self.x_bar,
            o_home: self.o_home + 1,
            x_home: self.x_home,
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut o_count = 0;
        let mut x_count = 0;

        for i in 0..24 {
            if self.o_points[i] > 0 && self.x_points[i] > 0 {
                return false;
            }

            o_count += self.o_points[i];
            x_count += self.x_points[i];
        }

        o_count += self.o_bar + self.o_home;
        x_count += self.x_bar + self.x_home;

        o_count == 15 && x_count == 15
    }

    /// The position obtained by swapping o's and x's.
    pub fn flip(&self) -> Position {
        let mut new_o_points = self.x_points;
        new_o_points.reverse();

        let mut new_x_points = self.o_points;
        new_x_points.reverse();

        Position {
            o_points: new_o_points,
            x_points: new_x_points,
            o_bar: self.x_bar,
            x_bar: self.o_bar,
            o_home: self.x_home,
            x_home: self.o_home,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔═══════════════════╤═══════════════════╗")?;

        for count in 0..5 {
            write!(f, "║")?;
            for point in 12..24 {
                if count == 4 && self.point_o_value(point) > 5 {
                    write!(f, " {} ", self.point_o_value(point))?;
                } else if count == 4 && self.point_x_value(point) > 5 {
                    write!(f, " {} ", self.point_x_value(point))?;
                } else if self.point_o_value(point) > count {
                    write!(f, " ● ")?;
                } else if self.point_x_value(point) > count {
                    write!(f, " ○ ")?;
                } else if count == 0 {
                    write!(f, " . ")?;
                } else {
                    write!(f, "   ")?;
                }
                if point == 17 {
                    write!(f, " │ ")?;
                }
            }
            write!(f, "║")?;

            if count == 0 && self.x_home > 0 {
                write!(f, " ○ {}", self.x_home)?;
            }

            writeln!(f)?;
        }

        write!(f, "║                   │                   ║")?;
        if self.x_bar_value() > 0 {
            write!(f, " bar: ○ {}", self.x_bar_value())?;
        }
        writeln!(f)?;

        write!(f, "║                   │                   ║")?;
        if self.o_bar_value() > 0 {
            write!(f, " bar: ● {}", self.o_bar_value())?;
        }
        writeln!(f)?;

        for count in (0..5).rev() {
            write!(f, "║")?;
            for point in (0..12).rev() {
                if count == 4 && self.point_o_value(point) > 5 {
                    write!(f, " {} ", self.point_o_value(point))?;
                } else if count == 4 && self.point_x_value(point) > 5 {
                    write!(f, " {} ", self.point_x_value(point))?;
                } else if self.point_o_value(point) > count {
                    write!(f, " ● ")?;
                } else if self.point_x_value(point) > count {
                    write!(f, " ○ ")?;
                } else if count == 0 {
                    write!(f, " . ")?;
                } else {
                    write!(f, "   ")?;
                }
                if point == 6 {
                    write!(f, " │ ")?;
                }
            }
            write!(f, "║")?;

            if count == 0 && self.o_home > 0 {
                write!(f, " ● {}", self.o_home)?;
            }

            writeln!(f)?;
        }
        write!(f, "╚═══════════════════╧═══════════════════╝")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_initial() -> () {
        let p1 = Position::initial();
        let p2 = p1.flip();

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_flip_twice() -> () {
        let p1 = Position::make(
            &[(6, 5), (8, 3), (13, 5), (24, 1)],
            &[(1, 2), (7, 2), (12, 2), (17, 2), (18, 2), (19, 5)],
            1,
            0,
            0,
            0,
        );

        assert_ne!(p1, p1.flip());
        assert_eq!(p1, p1.flip().flip());
    }
}
