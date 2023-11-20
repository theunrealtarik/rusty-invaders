use std::{
  collections::HashMap,
  time::{Duration, Instant},
};

pub struct Interval {
  handle: usize,
  duration: Duration,
  last_triggered: Instant,
  callback: Box<dyn Fn()>,
  enabled: bool,
}

pub struct IntervalsManager {
  intervals: HashMap<usize, Interval>,
}

impl Interval {
  fn new(handle: usize, duration: Duration, callback: Box<dyn Fn()>) -> Self {
    let now = Instant::now();

    Self {
      handle,
      duration,
      last_triggered: now,
      callback,
      enabled: true,
    }
  }
}

impl IntervalsManager {
  pub fn new() -> Self {
    Self {
      intervals: HashMap::new(),
    }
  }

  pub fn set_interval<F, T: 'static>(&mut self, duration: Duration, callback: F, state: T) -> usize
  where
    T: Copy,
    F: Fn(T) + 'static,
  {
    let handle = self.intervals.len();

    self.intervals.insert(
      handle,
      Interval::new(handle, duration, Box::new(move || callback(state))),
    );

    handle
  }

  pub fn get_interval(&mut self, handle: usize) -> Option<&Interval> {
    self.intervals.get(&handle)
  }

  pub fn update(&mut self) {
    let now = Instant::now();

    for interval in self.intervals.values_mut() {
      if !interval.enabled {
        continue;
      }

      if now.duration_since(interval.last_triggered) > interval.duration {
        (interval.callback)();
        interval.last_triggered = Instant::now();
      }
    }
  }
}
