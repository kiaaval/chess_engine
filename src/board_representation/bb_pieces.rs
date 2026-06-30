use std::fmt;

use std::ops::BitOr;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct BitBoard(pub u64);
impl BitBoard {
    pub fn empty() -> Self {
        BitBoard(0)
    }

    pub fn set(&mut self, sq: u8) {
        self.0 |= 1u64 << sq;
    }

    pub fn clear(&mut self, sq: u8) {
        self.0 &= !(1u64 << sq);
    }

    pub fn toggle(&mut self, sq: u8) {
        self.0 ^= 1u64 << sq
    }
}
impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{char} ");
            }
            writeln!(f);
        }
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct EverySide<T> {
    sides: [T; 2],
}
impl<T: Copy> EverySide<T> {
    pub fn new(value: T) -> Self {
        EverySide { sides: [value; 2] }
    }
}

#[derive(Clone, Copy)]
pub struct EveryPiece<T> {
    pub pieces: [T; 6],
}
impl<T: Copy> EveryPiece<T> {
    pub fn new(value: T) -> Self {
        EveryPiece { pieces: [value; 6] }
    }
}

#[derive(Clone, Copy)]
pub struct Occupancy {
    pub all: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
}
impl Occupancy {
    pub fn new() -> Self {
        Occupancy {
            all: BitBoard::empty(),
            white: BitBoard::empty(),
            black: BitBoard::empty(),
        }
    }

    pub fn from_parts(all: BitBoard, white: BitBoard, black: BitBoard) -> Self {
        Occupancy { all, white, black }
    }
}

impl<T> Index<Color> for EverySide<T> {
    type Output = T;
    fn index(&self, c: Color) -> &T {
        &self.sides[c as usize]
    }
}

impl<T> IndexMut<Color> for EverySide<T> {
    fn index_mut(&mut self, c: Color) -> &mut T {
        &mut self.sides[c as usize]
    }
}

impl<T> Index<Piece> for EveryPiece<T> {
    type Output = T;
    fn index(&self, p: Piece) -> &T {
        &self.pieces[p as usize]
    }
}

impl<T> IndexMut<Piece> for EveryPiece<T> {
    fn index_mut(&mut self, p: Piece) -> &mut T {
        &mut self.pieces[p as usize]
    }
}
