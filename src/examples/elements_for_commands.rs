#[cfg(test)]
mod tests {
    // *** START IMPORTS ***

    use bitris_commands::prelude::*;

// *** END IMPORTS ***

    #[test]
    fn bit_shapes() {
        let shapes = BitShapes::try_from(vec![Shape::T, Shape::I, Shape::O])
            .expect("Failed to create the bit shapes");
        assert_eq!(shapes.len(), 3);
        assert_eq!(shapes.to_vec(), vec![Shape::T, Shape::I, Shape::O]);
    }

    #[test]
    fn clipped_board() {
        use std::str::FromStr;

        let board = Board64::from_str("
            XXX.......
            XXX.......
            XXX.......
            XXX.......
        ").expect("Failed to create the board");
        let clipped_board = ClippedBoard::try_new(board, 4)
            .expect("Failed to create the board");
        assert_eq!(clipped_board.spaces(), 40);
    }

    #[test]
    fn shape_counter() {
        let counter = ShapeCounter::from(vec![Shape::T, Shape::T, Shape::I]);
        assert_eq!(counter[Shape::T], 2);
        assert_eq!(counter[Shape::I], 1);
        assert_eq!(counter[Shape::O], 0);
    }
}
