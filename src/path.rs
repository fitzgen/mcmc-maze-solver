use super::ErgodicAndSymmetric;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Path {
    pub moves: Vec<Option<Move>>,
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    North,
    East,
    South,
    West,
}

impl Move {
    fn arbitrary(rng: &mut impl Rng) -> Option<Self> {
        match rng.gen_range(0, 5) {
            0 => Some(Move::North),
            1 => Some(Move::East),
            2 => Some(Move::South),
            3 => Some(Move::West),
            4 => None,
            _ => unreachable!(),
        }
    }
}

impl ErgodicAndSymmetric for Path {
    fn candidate(&self, rng: &mut impl Rng) -> Self {
        let mut candidate = self.clone();
        match rng.gen_range(0, 4) {
            // Mutate existing move.
            0 => {
                if !candidate.moves.is_empty() {
                    let idx = rng.gen_range(0, candidate.moves.len());
                    candidate.moves[idx] = Move::arbitrary(rng);
                }
            }

            // Append new move.
            1 => {
                candidate.moves.push(Move::arbitrary(rng));
            }

            // Remove last move.
            2 => {
                candidate.moves.pop();
            }

            // Swap moves.
            3 => {
                if !candidate.moves.is_empty() {
                    let a = rng.gen_range(0, candidate.moves.len());
                    let b = rng.gen_range(0, candidate.moves.len());
                    candidate.moves.swap(a, b);
                }
            }

            _ => unreachable!(),
        }

        candidate
    }
}
