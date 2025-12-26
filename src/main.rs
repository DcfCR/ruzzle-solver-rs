mod board;
mod trie;

use crate::board::{BoardMask, Index4x4, RuzzleBoard};
use crate::trie::TrieNode;

fn main() {
    println!("Hello, world!");
}

type Path = Vec<Index4x4>;

fn solve(root: &TrieNode, board: &RuzzleBoard) -> Vec<Path> {
    let mut out: Vec<Path> = vec![];
    for idx in Index4x4::all_indices_within_bounds() {
        if let Some(child) = root.find_in_children(board[idx]) {
            let mut path = vec![];
            dfs(
                child,
                board,
                BoardMask::from(0u16),
                idx,
                &mut path,
                &mut out,
            );
        }
    }
    out
}

fn dfs(
    node: &TrieNode,
    board: &RuzzleBoard,
    visited: BoardMask,
    idx: Index4x4,
    path: &mut Path,
    out: &mut Vec<Path>,
) {
    let new_visited = visited.with_at(true, idx);
    let neighbours = idx.get_neighbouring().filter(|n_idx| !new_visited[*n_idx]);
    path.push(idx);
    if node.is_terminal {
        out.push(path.clone());
    }
    for n_idx in neighbours {
        if let Some(child) = node.find_in_children(board[n_idx]) {
            dfs(child, board, new_visited, n_idx, path, out);
        }
    }
    path.pop();
}




















