#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
} // (rank, file)

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }

        Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let abs_rank = (self.0.rank - other.0.rank).abs();
        let abs_file = (self.0.file - other.0.file).abs();
        abs_rank == 0 || // same rank
        abs_file == 0 || // same file
        abs_rank == abs_file // same diagonal
    }
}
