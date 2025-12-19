use std::ops::Index;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BoardIndex<const W: usize, const H: usize> {
    flattened: usize,
}

impl<const W: usize, const H: usize> BoardIndex::<W, H> {
    fn from_xy (x: usize, y: usize) -> Self {
        BoardIndex::<W, H>{flattened: x + W * y}
    }

    fn to_xy(self) -> (usize, usize) {
        (self.flattened % W, self.flattened / W)
    }

    fn get_neighbouring(&self) -> impl Iterator<Item = BoardIndex::<W, H>> {
        let (x, y) = self.to_xy();

        [ // usize::MAX is assumed to be much larger than W and H.
          // This means we can assume that wrapping_sub will always produce
          // results either (correctly) within the board's bounds or very far beyond... 
          (x.wrapping_sub(1), y.wrapping_sub(1)),   // 1 diagram: |1|2|3|
          (x,                 y.wrapping_sub(1)),   // 2          |4|¤|5|
          (x.wrapping_add(1), y.wrapping_sub(1)),   // 3          |6|7|8|
          (x.wrapping_sub(1),                 y),   // 4
          (x.wrapping_add(1),                 y),   // 5
          (x.wrapping_sub(1), y.wrapping_add(1)),   // 6
          (x,                 y.wrapping_add(1)),   // 7
          (x.wrapping_add(1), y.wrapping_add(1))    // 8
        ]
        .into_iter()
        .filter( move |&(neighbour_x, neighbour_y)| neighbour_x < W && neighbour_y < H )
        .map(|(nx, ny)| BoardIndex::<W, H>::from_xy(nx, ny))
    }
}

impl<const W: usize, const H: usize> fmt::Display for BoardIndex::<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.to_xy();
        write!(f, "BoardIndex::<{}, {}> ({}, {})", W, H, x, y)?;
        Ok(())
    }
}

type Index4x4 = BoardIndex<4, 4>;

struct Board4x4<T>([T; 16]);

#[derive(Debug)]
struct BoardFromStrError {}

impl FromStr for Board4x4<char> {
    type Err = BoardFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let arr: [char; 16] = chars.try_into().map_err(|_| BoardFromStrError {})?;
        Ok(Board4x4(arr))
    }
}

impl<T> Index<Index4x4> for Board4x4<T> {
    type Output = T;

    fn index(&self, idx: Index4x4) -> &Self::Output {
        &self.0[idx.flattened]
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

type RuzzleBoard = Board4x4<char>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_from_string() {
        let alphabet = "abcdefghijklmnop";
        let board = RuzzleBoard::from_str(alphabet).unwrap();
        let mut idxs = (0..=15).map(|n| Index4x4 {flattened: n});
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
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(0, 0)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(1, 0)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(2, 0)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(0, 1)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(2, 1)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(0, 2)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(1, 2)));
        assert_eq!(mid_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(2, 2)));
        assert_eq!(mid_neighbours.next(), None); 

        let top_left = BoardIndex::<3, 3>::from_xy(0, 0);
        let mut tl_neighbours = top_left.get_neighbouring();
        assert_eq!(tl_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(1, 0)));
        assert_eq!(tl_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(0, 1)));
        assert_eq!(tl_neighbours.next(), Some(BoardIndex::<3, 3>::from_xy(1, 1)));
        assert_eq!(tl_neighbours.next(), None);

    }
}
