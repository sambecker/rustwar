use pbr::ProgressBar;
use std::time::{ Duration };

pub struct Progress {
  progress_bar: ProgressBar<std::io::Stdout>
}

impl Progress {
  pub fn new(
    size: usize,
    label: &str,
    width_option: Option<usize>,
    duration_option: Option<u64>,
  ) -> Self {
    let mut progress_bar = ProgressBar::new(size as u64);
    progress_bar.message(label);
    progress_bar.set_width(width_option);
    if let Some(duration) = duration_option {
      progress_bar.set_max_refresh_rate(Some(Duration::from_millis(duration)));
    }
    Progress { progress_bar: progress_bar }
  }
  pub fn tick(&mut self) {
    self.progress_bar.inc();
  }
  pub fn finish(&mut self, message: &str) {
    self.progress_bar.finish_print(message);
    self.progress_bar.finish_println("");
  }
}
