#![cfg_attr(test, deny(warnings))]

#[doc(hidden)]
pub extern crate env_logger;

extern crate log;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use env_logger::fmt::{Color, Style, StyledValue, Formatter};
use log::{Level, Record};

pub fn init() {
    env_logger::builder().format(formatter).init();
}

pub fn init_timed() {
    env_logger::builder().format(timed_formatter).init();
}

/// Formatter used in env_logger builder
///
/// Formats output with colored level
pub fn formatter(f: &mut Formatter, record: &Record) -> std::io::Result<()> {
    use std::io::Write;
    let target = record.target();
    let max_width = max_target_width(target);

    let mut style = f.style();
    let level = colored_level(&mut style, record.level());

    let mut style = f.style();
    let target = style.set_bold(true).value(Padded {
        value: target,
        width: max_width,
    });

    writeln!(
        f,
        " {} {} > {}",
        level,
        target,
        record.args(),
    )
}

/// Formatter used in env_logger builder
///
/// Formats output with time and colored level
pub fn timed_formatter(f: &mut Formatter, record: &Record) -> std::io::Result<()> {
    use std::io::Write;
    let target = record.target();
    let max_width = max_target_width(target);

    let mut style = f.style();
    let level = colored_level(&mut style, record.level());

    let mut style = f.style();
    let target = style.set_bold(true).value(Padded {
        value: target,
        width: max_width,
    });

    let time = f.timestamp_millis();

    writeln!(
        f,
        " {} {} {} > {}",
        time,
        level,
        target,
        record.args(),
    )
}

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width = self.width)
    }
}

static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    if max_width < target.len() {
        MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
        target.len()
    } else {
        max_width
    }
}

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}
