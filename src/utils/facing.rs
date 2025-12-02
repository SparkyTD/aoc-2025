use crate::utils::position::Position;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Facing {
    East,
    South,
    West,
    North,
}

impl Facing {
    pub fn apply(&self, position: &Position) -> Position {
        match self {
            Facing::East => Position { x: position.x + 1, y: position.y },
            Facing::West => Position { x: position.x - 1, y: position.y },
            Facing::South => Position { x: position.x, y: position.y + 1 },
            Facing::North => Position { x: position.x, y: position.y - 1 },
        }
    }

    #[allow(dead_code)]
    pub fn try_apply(&self, position: &Position) -> Option<Position> {
        let new_position = match self {
            Facing::East => (position.x as i64 + 1, position.y as i64 ),
            Facing::West => (position.x as i64 - 1, position.y as i64 ),
            Facing::South => (position.x as i64, position.y as i64 + 1 ),
            Facing::North => (position.x as i64, position.y as i64 - 1 ),
        };

        if new_position.0 < 0 || new_position.1 < 0 {
            None
        } else {
            Some(Position::new(new_position.0 as usize, new_position.1 as usize))
        }
    }

    pub fn all() -> Vec<Facing> {
        vec![Facing::North, Facing::East, Facing::South, Facing::West]
    }

    pub fn adjacents(&self) -> Vec<Facing> {
        Facing::all()
            .into_iter()
            .filter(|f| *f != *self)
            .collect::<Vec<Facing>>()
    }

    pub fn opposite(&self) -> Facing {
        match self {
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
            Facing::North => Facing::South,
        }
    }
}