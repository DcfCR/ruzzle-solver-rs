use std::fmt;
use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoardIndex<const W: usize, const H: usize> {
    flattened: usize,
}

impl<const W: usize, const H: usize> BoardIndex<W, H> {
    pub fn all_indices_within_bounds() -> impl Iterator<Item = Self> {
        (0..W * H).map(|n| Self { flattened: n })
    }

    pub const fn from_xy(x: usize, y: usize) -> Self {
        Self {
            flattened: x + W * y,
        }
    }

    pub const fn to_xy(self) -> (usize, usize) {
        (self.flattened % W, self.flattened / W)
    }

    pub fn get_neighbouring(&self) -> impl Iterator<Item = Self> {
        let (x, y) = self.to_xy();

        [
            // usize::MAX is assumed to be much larger than W and H.
            // This means we can assume that wrapping_sub will always produce
            // results either (correctly) within the board's bounds or very far beyond...
            (x.wrapping_sub(1), y.wrapping_sub(1)), // 1 diagram: |1|2|3|
            (x, y.wrapping_sub(1)),                 // 2          |4|¤|5|
            (x.wrapping_add(1), y.wrapping_sub(1)), // 3          |6|7|8|
            (x.wrapping_sub(1), y),                 // 4
            (x.wrapping_add(1), y),                 // 5
            (x.wrapping_sub(1), y.wrapping_add(1)), // 6
            (x, y.wrapping_add(1)),                 // 7
            (x.wrapping_add(1), y.wrapping_add(1)), // 8
        ]
        .into_iter()
        .filter(move |&(neighbour_x, neighbour_y)| neighbour_x < W && neighbour_y < H)
        .map(|(nx, ny)| Self::from_xy(nx, ny))
    }
}

impl<const W: usize, const H: usize> fmt::Display for BoardIndex<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.to_xy();
        write!(f, "BoardIndex::<{}, {}> ({}, {})", W, H, x, y)?;
        Ok(())
    }
}

pub type Index4x4 = BoardIndex<4, 4>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board4x4<T>([T; 16]);
// If const generic expressions were supported in Rust as of
// I would implement a generic Board class with const param-
// eters W and H (To match the BoardIndex<W,H>), and set the
// size of the array to W*H. Unfortunately, Rust does not
// allow this at the time of writing (2025-12-21). It exists
// in nightly builds, so it might be coming.

impl<T: Copy> Board4x4<T> {
    pub fn with_at(&self, value: T, idx: Index4x4) -> Self {
        let mut arr = self.0.clone();
        arr[idx.flattened] = value;
        Board4x4::<T>(arr)
    }
}

impl From<&str> for Board4x4<char> {
    fn from(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let arr: [char; 16] = chars.try_into().unwrap();
        Board4x4(arr)
    }
}

impl From<u16> for Board4x4<bool> {
    fn from(u: u16) -> Self {
        // Bit order: most significant bit  => top left
        //            least significant bit => bottom right
        let arr: [bool; 16] = std::array::from_fn(|n| ((u >> (15 - n)) & 1) != 0);
        Board4x4(arr)
    }
}

impl<T> Index<Index4x4> for Board4x4<T> {
    // 2D ("Grid") indexing.
    type Output = T;

    fn index(&self, idx: Index4x4) -> &Self::Output {
        &self.0[idx.flattened]
    }
}

impl<T> Index<usize> for Board4x4<T> {
    // 1D ("Flat") indexing.
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T: fmt::Display> fmt::Display for Board4x4<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.chunks(4) {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub type RuzzleBoard = Board4x4<char>;
pub type BoardMask = Board4x4<bool>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_from_string() {
        let alphabet = "abcdefghijklmnop";
        let board = RuzzleBoard::from(alphabet);
        let mut idxs = (0..=15).map(|n| Index4x4 { flattened: n });
        assert_eq!(board[idxs.next().unwrap()], 'a');
        assert_eq!(board[idxs.next().unwrap()], 'b');
        assert_eq!(board[idxs.next().unwrap()], 'c');
        assert_eq!(board[idxs.next().unwrap()], 'd');
        assert_eq!(board[idxs.next().unwrap()], 'e');
        assert_eq!(board[idxs.next().unwrap()], 'f');
        assert_eq!(board[idxs.next().unwrap()], 'g');
        assert_eq!(board[idxs.next().unwrap()], 'h');
        assert_eq!(board[idxs.next().unwrap()], 'i');
        assert_eq!(board[idxs.next().unwrap()], 'j');
        assert_eq!(board[idxs.next().unwrap()], 'k');
        assert_eq!(board[idxs.next().unwrap()], 'l');
        assert_eq!(board[idxs.next().unwrap()], 'm');
        assert_eq!(board[idxs.next().unwrap()], 'n');
        assert_eq!(board[idxs.next().unwrap()], 'o');
        assert_eq!(board[idxs.next().unwrap()], 'p');
    }

    #[test]
    fn get_neighbouring() {
        let middle = BoardIndex::<3, 3>::from_xy(1, 1);
        let mut mid_neighbours = middle.get_neighbouring();
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(0, 0))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(1, 0))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(2, 0))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(0, 1))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(2, 1))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(0, 2))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(1, 2))
        );
        assert_eq!(
            mid_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(2, 2))
        );
        assert_eq!(mid_neighbours.next(), None);

        let top_left = BoardIndex::<3, 3>::from_xy(0, 0);
        let mut tl_neighbours = top_left.get_neighbouring();
        assert_eq!(
            tl_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(1, 0))
        );
        assert_eq!(
            tl_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(0, 1))
        );
        assert_eq!(
            tl_neighbours.next(),
            Some(BoardIndex::<3, 3>::from_xy(1, 1))
        );
        assert_eq!(tl_neighbours.next(), None);
    }
}
