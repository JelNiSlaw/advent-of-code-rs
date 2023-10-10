pub const fn calculate_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    if x_diff.abs() > 1 || y_diff.abs() > 1 {
        (x_diff.signum(), y_diff.signum())
    } else {
        (0, 0)
    }
}
