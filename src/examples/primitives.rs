#[cfg(test)]
mod tests {
    // *** START IMPORTS ***

    use bitris::macros::*;
    use bitris::prelude::*;

// *** END IMPORTS ***

    #[test]
    fn make_pieces() {
        let piece = Piece { shape: Shape::T, orientation: Orientation::North };

        let from_new = Piece::new(Shape::T, Orientation::North);
        assert_eq!(piece, from_new);

        let use_with = Shape::T.with(Orientation::North);
        assert_eq!(piece, use_with);

        let use_macro = piece!(TN);
        assert_eq!(piece, use_macro);
    }

    #[test]
    fn piece_canonical() {
        assert_eq!(piece!(OS).canonical(), Some(piece!(ON)));
        assert_eq!(piece!(OE).canonical(), Some(piece!(ON)));
        assert_eq!(piece!(OW).canonical(), Some(piece!(ON)));
        assert_eq!(piece!(ON).canonical(), None);
    }

    #[test]
    fn cc_placement() {
        // ..........
        // ..#.......
        // .#@#......  << @ is cc(2,2)
        // ..........
        // ..........

        let placement = CcPlacement {
            piece: Piece::new(Shape::T, Orientation::North),
            position: CcPosition { cx: 2, cy: 2 },
        };
        assert_eq!(placement.piece.shape, Shape::T);
        assert_eq!(placement.piece.orientation, Orientation::North);
        assert_eq!(placement.position.cx, 2);
        assert_eq!(placement.position.cy, 2);

        // 他の書き方
        let use_with = piece!(TN).with(cc(2, 2));
        assert_eq!(placement, use_with);
    }

    #[test]
    fn bl_placement() {
        // ..........
        // ..#.......
        // .@##......  << @ is bl(1,2)
        // ..........
        // ..........

        let placement = BlPlacement {
            piece: Piece::new(Shape::T, Orientation::North),
            position: BlPosition { lx: 1, by: 2 },
        };
        assert_eq!(placement.piece.shape, Shape::T);
        assert_eq!(placement.piece.orientation, Orientation::North);
        assert_eq!(placement.position.lx, 1);
        assert_eq!(placement.position.by, 2);

        // 他の書き方
        let use_with = piece!(TN).with(bl(1, 2));
        assert_eq!(placement, use_with);
    }

    #[test]
    fn tr_placement() {
        // ..........
        // ..#@......
        // .###......  << @ is tr(3,3)
        // ..........
        // ..........

        let placement = TrPlacement {
            piece: Piece::new(Shape::T, Orientation::North),
            position: TrPosition { rx: 3, ty: 3 },
        };
        assert_eq!(placement.piece.shape, Shape::T);
        assert_eq!(placement.piece.orientation, Orientation::North);
        assert_eq!(placement.position.rx, 3);
        assert_eq!(placement.position.ty, 3);

        // 他の書き方
        let use_with = piece!(TN).with(tr(3, 3));
        assert_eq!(placement, use_with);
    }

    #[test]
    fn placement_conversion() {
        // ..........
        // ..#.......
        // .###......
        // ..........
        // ..........

        let cc_placement = piece!(TN).with(cc(2, 2));
        let bl_placement = piece!(TN).with(bl(1, 2));
        let tr_placement = piece!(TN).with(tr(3, 3));

        assert_eq!(cc_placement, bl_placement.to_cc_placement());
        assert_eq!(cc_placement, tr_placement.to_cc_placement());
    }

    #[test]
    fn rotated_placements() {
        // Cwに回転
        // ..........    ..........
        // ..#.......    ..#.......
        // .#@#...... => ..@#......  << @ is cc
        // ..........    ..#.......
        // ..........    ..........
        let cc_placement = piece!(TN).with(cc(2, 2));
        assert_eq!(cc_placement.rotate(Rotation::Cw), piece!(TE).with(cc(2, 2)));

        // 結果は同じだけど、座標の表現はかわらない
        // ..........    ..........
        // ..#.......    ..#.......
        // .@##...... => ..##......  << @ is bl
        // ..........    ..@.......
        // ..........    ..........
        let bl_placement = piece!(TN).with(bl(1, 2));
        assert_eq!(bl_placement.rotate(Rotation::Cw), piece!(TE).with(bl(2, 1)));

        // ..........    ..........
        // ..........    ..#@......
        // .##@...... => ..##......  << @ is tr
        // ..#.......    ..#.......
        // ..........    ..........
        let tr_placement = piece!(TS).with(tr(3, 2));
        assert_eq!(tr_placement.rotate(Rotation::Ccw), piece!(TE).with(tr(3, 3)));

        // 他の書き方
        assert_eq!(cc_placement.cw(), cc_placement.rotate(Rotation::Cw));
        assert_eq!(cc_placement.ccw(), cc_placement.rotate(Rotation::Ccw));
        assert_eq!(cc_placement.r180(), cc_placement.rotate(Rotation::R180));
    }

    #[test]
    fn piece_blocks() {
        let piece_blocks = piece!(TN).to_piece_blocks();
        assert_eq!(piece_blocks.shape(), Shape::T);
        assert_eq!(piece_blocks.orientation(), Orientation::North);
        assert_eq!(piece_blocks.width, 3);
        assert_eq!(piece_blocks.height, 2);
        assert_eq!(piece_blocks.offsets, [dd(-1, 0), dd(0, 0), dd(1, 0), dd(0, 1)]);
    }

    #[test]
    fn piece_blocks_to_locations() {
        let piece_blocks = piece!(TN).to_piece_blocks();

        // location from cc
        let cc_position = cc(2, 2);
        assert_eq!(
            piece_blocks.to_locations(cc_position),
            [xy(1, 2), xy(2, 2), xy(3, 2), xy(2, 3)],
        );

        // location from bl
        let bl_position = bl(1, 2);
        assert_eq!(
            piece_blocks.to_locations(cc_position),
            piece_blocks.to_locations(bl_position.to_cc_position(piece_blocks)),
        );
    }
}
