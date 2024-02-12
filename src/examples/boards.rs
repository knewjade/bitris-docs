#[cfg(test)]
mod tests {
    // *** START IMPORTS ***

    use bitris::macros::*;
    use bitris::prelude::*;

// *** END IMPORTS ***

    #[test]
    fn make_boards() {
        // Make boards. Select the bit width.
        let board8: Board8 = Board8::blank();
        let board16: Board16 = Board16::blank();
        let board32: Board32 = Board32::blank();
        let board64: Board64 = Board64::blank();

        // Depending on the bit width, the ceiling height will vary.
        assert_eq!(board8.ceiling(), 8);
        assert_eq!(board16.ceiling(), 16);
        assert_eq!(board32.ceiling(), 32);
        assert_eq!(board64.ceiling(), 64);
    }

    #[test]
    fn make_board_from_str() {
        use std::str::FromStr;

        // Boards can be created from strings.
        let board = Board64::from_str("
            ..........
            ####....##
            ####...###
            ####..####
            ####...###
        ").expect("Failed to create the board");
        assert_eq!(board.count_blocks(), 28);
        assert_eq!(board.well_top(), 4);
    }

    #[test]
    fn set_and_unset() {
        let mut board = Board64::blank();

        // Set and unset a block. Manipulation of the board changes itself (it means mutable).
        // ..........
        // ..........
        // ..........
        // ..#.......
        // ..........
        board.set_at(xy(2, 1));
        assert_eq!(board.count_blocks(), 1);
        assert!(!board.is_empty());

        assert!(board.is_occupied_at(xy(2, 1)));
        assert!(!board.is_free_at(xy(2, 1)));

        // ..........
        // ..........
        // ..........
        // ..........
        // ..........
        board.unset_at(xy(2, 1));
        assert_eq!(board.count_blocks(), 0);
        assert!(board.is_empty());

        assert!(!board.is_occupied_at(xy(2, 1)));
        assert!(board.is_free_at(xy(2, 1)));
    }

    #[test]
    fn place_on() {
        use std::str::FromStr;

        let mut board = Board64::from_str("
            ..........
            ........##
            ........##
        ").expect("Failed to create the board");

        // 成功した場合
        let placement = piece!(ON).with(bl(0, 0));
        let succeed = placement.place_on(&mut board);
        assert!(succeed);

        assert_eq!(board, Board64::from_str("
            ..........
            ##......##
            ##......##
        ").expect("Failed to create the board"));

        // すでにブロックがあると配置できない. フィールドは変化しない
        let placement = piece!(ON).with(bl(0, 0));
        let updated = placement.place_on(&mut board);
        assert!(!updated);

        // フィールド外は配置できない
        let placement = piece!(ON).with(bl(-1, 0));
        let updated = placement.place_on(&mut board);
        assert!(!updated);

        // 空中には配置できない
        let placement = piece!(ON).with(bl(5, 5));
        let updated = placement.place_on(&mut board);
        assert!(!updated);

        // 事前チェック用
        let placement = piece!(ON).with(bl(4, 0));
        assert!(placement.can_place_on(&board));
        assert!(placement.is_landing(&board));
        assert!(placement.is_in_free_space(&board));
    }

    #[test]
    fn place_on_and_clear_lines() {
        use std::str::FromStr;

        let mut board = Board64::from_str("
            ..........
            ....######
            ....######
        ").expect("Failed to create the board");

        let o_north = piece!(ON);

        let lines_cleared = o_north.with(bl(0, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::blank()));

        assert_eq!(board, Board64::from_str("
            ..........
            ##..######
            ##..######
        ").expect("Failed to create the board"));

        let lines_cleared = o_north.with(bl(2, 0)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, Some(Lines::new(0b11)));
        if let Some(lines) = lines_cleared {
            assert_eq!(lines.count(), 2);
        }

        assert!(board.is_empty());

        // Access off board
        let lines_cleared = o_north.with(bl(-1, -1)).place_on_and_clear_lines(&mut board);
        assert_eq!(lines_cleared, None);
    }

    #[test]
    fn convert() {
        // If you want to extend the bit width, you can use `into()` to convert.
        let mut board32: Board32 = Board8::blank().into();
        board32.set_at(xy(0, 8));
        assert_eq!(board32.ceiling(), 32);
        assert_eq!(board32.count_blocks(), 1);

        // If you want to shrink the bit width, you can use `shrink_from()` to convert.
        // 外にあるブロックは切り捨て
        let board8: Board8 = Board8::shrink_from(board32);
        assert_eq!(board8.ceiling(), 8);
        assert_eq!(board8.count_blocks(), 0);
    }
}
