use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let size = autopilot::screen::size();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(rng.random_range(60000..600000)));
        autopilot::mouse::smooth_move(
            autopilot::geometry::Point {
                x: rng.random_range(0.0..size.width),
                y: rng.random_range(0.0..size.height),
            }, None
        ).expect("Failed to move mouse");
    }
}
