//! Small but flexible "in-place" progress bar. Styleable, resizable.
use std::io::Write;

/// default progress bar size (width)
const _DEFAULT_WIDTH: u32 = 20;

pub struct Oxibar {
    /// how many items do we have
    total: u32,

    /// current progress (current item)
    current: u32,

    /// progress bar width
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

impl Default for Oxibar {
    fn default() -> Self {
        Self {
            total: 0,
            current: 0,
            size: _DEFAULT_WIDTH,
            style_empty: "-".to_string(),
            style_filled: "=".to_string(),
            style_cursor: ">".to_string(),
            show_percentage: true,
            show_items: true,
        }
    }
}

impl Oxibar {
    /// Create new progress bar for `total` items, starting current progress from 0.
    /// 
    /// This total number will represent 100% progress
    pub fn new(total: u32) -> Oxibar {
        Oxibar {
            total,
            ..Default::default()
        }
    }

    pub fn set_size(&mut self, size: u32) -> &mut Self {
        self.size = size;

        self
    }

    pub fn show_percentage(&mut self, flag: bool) -> &mut Self {
        self.show_percentage = flag;

        self
    }

    pub fn show_items(&mut self, flag: bool) -> &mut Self {
        self.show_items = flag;

        self
    }

    /// Set characters to represent empty progress.
    /// 
    /// Example: if minus (-) char used – progress will look like this: `[--------]`
    pub fn set_style_empty(&mut self, style: &str) -> &mut Self {
        self.style_empty = style.to_string();

        self
    }

    /// Set characters to represent completed part of progress, filled bar.
    /// 
    /// Example: if equal (=) char used – progress will look like this: `[===>----]`
    pub fn set_style_filled(&mut self, style: &str) -> &mut Self {
        self.style_filled = style.to_string();

        self
    }

    /// Set characters to represent current position of progress.
    /// 
    /// Example: if plus (+) char used – progress will look like this: `[===+----]`
    pub fn set_style_cursor(&mut self, style: &str) -> &mut Self {
        self.style_cursor = style.to_string();

        self
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
    fn calculate_values(&self, total: u32, current: u32) -> (u32, u32, u32) {
        let total = total as f32;
        let _current = current as f32;

        let multiplier = 100.0 / self.size as f32;
        let rate: f32 = self.size as f32 / total;
        let rel_curr = rate * current as f32;
        let percent = rel_curr * multiplier;

        (self.size, rel_curr.floor() as u32, percent.ceil() as u32)
    }

    /// Increase number of processed items by one. Useful in per-item progress loop
    pub fn advance(&mut self) -> &Self {
        self.current += 1;

        self
    }
    /// Increase number of processed items by defined value.
    /// 
    /// Useful for batch progress updating
    pub fn advance_multiple(&mut self, num: u32) -> &Self {
        self.current += num;

        self
    }

    /// Print current progress on the same line overwritting previous progress.
    /// 
    /// This methong use `\r` escape sequence, which is supported by the most terminal.
    /// 
    /// _Please note:_ printing anything between `print()` calls will result in broken output
    pub fn print(&self) {
        let (p_total, p_curr, percent) = self.calculate_values(self.total, self.current);
        let left = p_total - p_curr;

        let cursor = match p_curr < p_total {
            true => &self.style_cursor,
            false => &self.style_filled,
        };

        let c_done: String = match p_curr > 0 {
            true => self.style_filled.repeat((p_curr - 1) as usize),
            false => String::from(""),
        };

        let c_remain = &self.style_empty.repeat(left as usize);

        let progress = format!("[{}{}{}] {}/100%", c_done, cursor, c_remain, percent);
        print!("\r{} ({}/{})", progress, self.current, self.total);
        std::io::stdout().flush().unwrap();

        if p_total == p_curr {
            println!("\x20");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::components::progressbar::Oxibar;

    #[test]
    fn progress_calculate_with_10() {
        const WIDTH: u32 = 10;
        let mut progress_bar = Oxibar::new(0);
        progress_bar.set_size(WIDTH);

        assert_eq!(progress_bar.calculate_values(4, 2), (WIDTH, 5, 50));

        assert_eq!(progress_bar.calculate_values(5, 3), (WIDTH, 6, 60));
        assert_eq!(progress_bar.calculate_values(5, 5), (WIDTH, 10, 100));

        assert_eq!(progress_bar.calculate_values(10, 4), (WIDTH, 4, 40));
        assert_eq!(progress_bar.calculate_values(10, 9), (WIDTH, 9, 90));
        assert_eq!(progress_bar.calculate_values(10, 10), (WIDTH, 10, 100));

        assert_eq!(progress_bar.calculate_values(45, 10), (WIDTH, 2, 23));
        assert_eq!(progress_bar.calculate_values(45, 12), (WIDTH, 2, 27));

        assert_eq!(progress_bar.calculate_values(546, 180), (WIDTH, 3, 33));
    }

    #[test]
    fn print_some_progress() {
        let total = 10;
        let mut progress_bar = Oxibar::new(total);

        for _ in 0..total {
            progress_bar.advance().print();

            // it will slow tests down, but it helps to visually inspect the progress
            thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn print_some_styled() {
        let total = 10;
        let mut progress_bar = Oxibar::new(total);

        progress_bar
            .set_size(30)
            .set_style_empty("~")
            .set_style_cursor("+")
            .set_style_filled("#");

        for i in 0..total {
            progress_bar.advance().print();
            assert_eq!(progress_bar.current, i + 1);

            // it will slow tests down, but it helps to visually inspect the progress
            thread::sleep(Duration::from_millis(100));
        }
    }
}
