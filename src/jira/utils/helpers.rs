use cursive::Cursive;

pub fn calculate_view_size(
    cursive: &mut Cursive,
    numerator: usize,
    denominator: usize,
) -> (usize, usize) {
    let screen_size = cursive.screen_size();
    (
        screen_size.x * numerator / denominator,
        screen_size.y * numerator / denominator,
    )
}
