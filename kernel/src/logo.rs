
/// .______      __    __ .____    __      .<----_________ -------_________
/// |   _  \    |  |  |  ||    \  |  |       .<---_/ _____ \  __-- / _____/
/// |  |_)  |   |  |  |  ||  .  \ |  |      .<--- /  /    \ \  ---|  (
/// |      /    |  |  |  ||  |\  \|  |     .<--- (  (      ) | _---\  \
/// |  |\  \----|  `--'  ||  | \  '  |      .<---_\__\____/ / ______) /
/// | _| `._____|\______/ |__|  \____|     .<---___\______ /_________/

use owo_colors::OwoColorize;

pub const LOGO: &str = include_str!("logo.txt");
pub fn show() {
    println!("{}", LOGO.yellow());
}