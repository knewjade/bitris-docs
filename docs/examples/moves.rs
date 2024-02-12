#[cfg(test)]
mod tests {
    // *** START IMPORTS ***

    use bitris::macros::*;
    use bitris::prelude::*;

// *** END IMPORTS ***

    #[test]
    fn generate() {
        use std::str::FromStr;

        let board = Board64::from_str("
            ..........
            ......####
            .....#####
            ....######
            ...#######
            ..########
            .#########
        ").expect("Failed to create the board");

        let spawn = piece!(IN).with(cc(4, 20)).to_bl_placement();

        // Specify the rotation system and a drop type. The default is selected SRS & softdrop.
        // You can specify your own rotation system.
        let move_rules = MoveRules::default();
        // OR `let move_rules = MoveRules::srs(AllowMove::Softdrop);`

        let all_moves = move_rules.generate_all_moves(board, spawn);
        assert_eq!(all_moves.len(), 34);

        let minimized_moves = move_rules.generate_minimized_moves(board, spawn);
        assert_eq!(minimized_moves.len(), 17);

        // The result includes both orientations that have the same form.
        assert!(all_moves.contains(&piece!(IN).with(bl(0, 3))));
        assert!(all_moves.contains(&piece!(IS).with(bl(0, 3))));

        // The result includes one orientation that has the same form.
        assert!(minimized_moves.contains(&piece!(IN).with(bl(0, 3))));
        assert!(!minimized_moves.contains(&piece!(IS).with(bl(0, 3))));
    }

    #[test]
    fn reach() {
        use std::str::FromStr;

        let board = Board64::from_str("
            .##..#####
            .#..######
        ").expect("Failed to create the board");

        let move_rules = MoveRules::srs(AllowMove::Softdrop);

        let spawn = piece!(SN).with(bl(4, 20));

        // S-South is reachable using softdrop.
        assert!(move_rules.can_reach(piece!(SS).with(bl(2, 0)), board, spawn));
        assert!(move_rules.can_reach_strictly(piece!(SS).with(bl(2, 0)), board, spawn));

        // S-North
        assert!(move_rules.can_reach(piece!(SN).with(bl(2, 0)), board, spawn));
        assert!(!move_rules.can_reach_strictly(piece!(SN).with(bl(2, 0)), board, spawn));
    }

    #[test]
    fn custom_rotation_system() {
        use std::slice::Iter;

        struct MyKickTable;

        // "Oは回転しない""そのほかは中心を軸に回転するがキックはしない"Rotation System
        impl RotationSystem for MyKickTable {
            fn iter_kicks(&self, piece: Piece, _: Rotation) -> Iter<'_, Kick> {
                const KICK: [Kick; 1] = [Kick::new(Offset::new(0, 0))];
                match piece.shape {
                    Shape::O => [].iter(), // Cannot rotate
                    _ => KICK.iter(), // Rotatable, but no kick
                }
            }

            fn is_moving_in_rotation(&self, shape: Shape) -> bool {
                // Oは回転できないのでフォームも常に変わらない。したがって唯一 `false` を返す
                shape != Shape::O
            }
        }

        let rotation_system = MyKickTable;
        let move_rules = MoveRules::new(&rotation_system, AllowMove::Softdrop);

        // 以降の使い方はSRSと同じ

        let board = Board64::blank();
        let spawn = piece!(IN).with(cc(4, 20)).to_bl_placement();
        let all_moves = move_rules.generate_all_moves(board, spawn);
        assert_eq!(all_moves.len(), 34);
    }
}
