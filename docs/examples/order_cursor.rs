#[cfg(test)]
mod tests {
    // *** START IMPORTS ***
    use bitris::prelude::*;

// *** END IMPORTS ***

    #[test]
    fn order_cursor() {
        use Shape::*;

        let shapes = vec![O, S, T, I];
        let cursor = OrderCursor::from(&shapes);

        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 4);

        // --- [](O)STI
        assert_eq!(cursor.first(), Some(&O));
        assert_eq!(cursor.second(), Some(&S));
        // ---

        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&O));
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 3);

        // --- [](S)TI
        assert_eq!(cursor.first(), Some(&S));
        assert_eq!(cursor.second(), Some(&T));
        // ---

        let (shape, cursor) = cursor.pop(PopOp::Second);
        assert_eq!(shape, Some(&T));
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 2);

        // --- [S](I)
        assert_eq!(cursor.first(), Some(&S));
        assert_eq!(cursor.second(), Some(&I));
        // ---

        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&S));
        assert!(cursor.has_next());
        assert_eq!(cursor.len_remaining(), 1);

        // --- [I]()
        assert_eq!(cursor.first(), Some(&I));
        assert_eq!(cursor.second(), None);
        // ---

        let (shape, cursor) = cursor.pop(PopOp::First);
        assert_eq!(shape, Some(&I));
        assert!(!cursor.has_next());
        assert_eq!(cursor.len_remaining(), 0);

        // --- []()
        assert_eq!(cursor.first(), None);
        assert_eq!(cursor.second(), None);
        // ---
    }

    #[test]
    fn decide_next_op() {
        use Shape::*;

        let shapes = vec![O, S, T, I];
        let cursor = OrderCursor::from(&shapes);

        assert_eq!(cursor.decide_next_op(&O), Some(PopOp::First));
        assert_eq!(cursor.decide_next_op(&S), Some(PopOp::Second));
        assert_eq!(cursor.decide_next_op(&T), None);
    }
}
