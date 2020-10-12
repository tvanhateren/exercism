pub struct ChessPosition {
    file: i32,
    rank: i32,
}

impl ChessPosition {
    pub fn new(file: i32, rank: i32) -> Option<ChessPosition> {
        if file < 0 || rank < 0 || file > 7 || rank > 7 {
            None
        } else {
            Some(ChessPosition { file, rank })
        }
    }
}

pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { pos }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.file == other.pos.file
            || self.pos.rank == other.pos.rank
            || (self.pos.file - other.pos.file).abs() == (self.pos.rank - other.pos.rank).abs()
    }
}
