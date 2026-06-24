use std::ops::{Index, IndexMut};

pub enum Color {
    White = 0,
    Black = 1,
}

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
    pieces: [T; 6],
}
impl<T: Copy> EveryPiece<T> {
    pub fn new(value: T) -> Self {
        EveryPiece { pieces: [value; 6] }
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
