use std::io::Write;

const _DEFAULT_WIDTH: u32 = 20;

struct Oxibar {
    /// how many items do we have
    total: u32,

    /// progress bar width in chars
    size: u32,

    /// visual represntation of the empty progress barr cells (default: `-`)
    style_empty: String,

    /// visual represntation of the filled (progressed) progress barr cells (default: `=`)
    style_filled: String,

    /// visual represntation of cursor (current items on the bar) (default: `>`)
    style_cursor: String,

    /// show progresss in percentc (for e.g.: `42/100%`)
    show_percentage: bool,

    /// show progresss in processed items to total (for e.g.: `76/345`)
    show_items: bool,
}

impl Oxibar {
    fn new(total: u32) -> Oxibar {
        Oxibar {
            total,
            ..Default::default()
        }
    }

    fn show_percentage(&mut self, flag: bool) -> &Self {
        self.show_percentage = flag;

        self
    }

    fn show_items(&mut self, flag: bool) -> &Self {
        self.show_items = flag;

        self
    }

    fn set_style_empty(&mut self, style: &str) -> &Self{
        self.style_empty = style.to_string();

        self
    }
    fn set_style_filled(&mut self, style: &str) -> &Self{
        self.style_filled = style.to_string();

        self
    }
    fn set_style_cursor(&mut self, style: &str) -> &Self{
        self.style_cursor = style.to_string();

        self
    }
}

impl Default for Oxibar {
    fn default() -> Self {
        Self {
            total: 0,
            size: _DEFAULT_WIDTH,
            style_empty: "-".to_string(),
            style_filled: "=".to_string(),
            style_cursor: ">".to_string(),
            show_percentage: true,
            show_items: true,
        }
    }
}

/// Calculate 3 values: total cells, complete cells and progress percenatage
///
/// # Arguments
/// * `total` - total record in progress bar
/// * `current` - current record being processed
///
/// # Return
/// Tuple `(a, b, c)`, where:
///  * `a` - total cells, actual progress bar width in chars)
///  * `b` - (floored) processed cells, how many cells should be filled-out as "done" (in chars).
///  * `c` - (ceiled) percentage representation of current progress, for e.g.: `14` means `14%`
///
///
/// Example:
///
/// With progress bar size of 10 chars, total records of 45 (`total = 45`)
/// and current progress of 10 (`current = 10`), will return:
///
///  `(10, 2, 23)`, where `(total_cells, relative_cells_progress, percent_progress)`
fn calculate_values(total: u32, current: u32) -> (u32, u32, u32) {
    let _total = total as f32;
    let _current = current as f32;

    let multiplier = 100 / _DEFAULT_WIDTH;
    let rate: f32 = _DEFAULT_WIDTH as f32 / _total;
    let rel_curr = rate * current as f32;
    let percent = rel_curr * multiplier as f32;

    (
        _DEFAULT_WIDTH,
        rel_curr.floor() as u32,
        percent.round() as u32,
    )
}

pub fn print_update_progress(total: u32, current: u32) {
    let (p_total, p_curr, percent) = calculate_values(total, current);
    let left = p_total - p_curr;

    let cursor = match p_curr < p_total {
        true => ">",
        false => "=",
    };

    let progress = format!(
        "[{}{}{}] {}/100%",
        "=".repeat((p_curr - 1) as usize),
        cursor,
        "-".repeat(left as usize),
        percent
    );
    print!("\r{} ({}/{})", progress, current, total);
    std::io::stdout().flush().unwrap();

    if p_total == p_curr {
        println!("\x20");
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::progress::calculate_values;

    use super::print_update_progress;

    #[test]
    fn progress_calculate() {
        assert_eq!(calculate_values(4, 2), (10, 5, 50));

        assert_eq!(calculate_values(5, 3), (10, 6, 60));
        assert_eq!(calculate_values(5, 5), (10, 10, 100));

        assert_eq!(calculate_values(10, 4), (10, 4, 40));
        assert_eq!(calculate_values(10, 10), (10, 10, 100));

        assert_eq!(calculate_values(45, 10), (10, 2, 22));
        assert_eq!(calculate_values(45, 12), (10, 2, 27));

        assert_eq!(calculate_values(546, 180), (10, 3, 33));
    }

    #[test]
    fn print_some_progress() {
        let total = 10;

        for i in 0..total {
            print_update_progress(total, i+1);
            thread::sleep(Duration::from_millis(500));
        }
    }
}
