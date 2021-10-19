use pbr::MultiBar;
use std::time::{ Duration };

const DEFAULT_REFRESH_RATE:u64 = 250;

pub struct ProgressMulti {
  progress_bars: MultiBar<std::io::Stdout>,
  width_option: Option<usize>,
  duration_option: Option<u64>,
}

impl ProgressMulti {
  pub fn new(
    width_option: Option<usize>,
    duration_option: Option<u64>,
  ) -> Self {
    ProgressMulti {
      progress_bars: MultiBar::new(),
      width_option: width_option,
      duration_option: duration_option,
    }
  }
  pub fn new_bar(
    &self,
    label: &str,
    size: usize,
  ) -> impl FnMut(Option<&str>) {
    let mut bar = self.progress_bars.create_bar(size as u64);

    bar.message(label);
    bar.set_width(self.width_option);
    bar.set_max_refresh_rate(Some(Duration::from_millis(
      self.duration_option.unwrap_or(DEFAULT_REFRESH_RATE))));

    let increment_or_finish_callback = move |finish_message: Option<&str>| {
      if let Some(message) = finish_message {
        bar.finish_println(message);
      } else {
        bar.inc();
      }
    };

    increment_or_finish_callback
  }
  pub fn start(&mut self) {
    self.progress_bars.listen();
  }
}
