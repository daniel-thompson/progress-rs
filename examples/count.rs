use progress::*;

fn main() {
    // In order to prevent this progress bar completing instantly
    // we rate limit the iterator so each count takes at least 100ms
    for _ in (0..113)
        .rate_limit(std::time::Duration::from_millis(100))
        .show_percent()
    {}
}
