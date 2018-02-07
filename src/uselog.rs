use std::sync::{Arc, Mutex};
use log::{Level, LevelFilter, Log, Record, Metadata};
use log ;

#[cfg(feature = "std")]
use log::set_boxed_logger;

#[cfg(not(feature = "std"))]
fn set_boxed_logger(logger: Box<Log>) -> Result<(), log::SetLoggerError> {
    log::set_logger(unsafe { &*Box::into_raw(logger) })
}

struct State {
    last_log: Mutex<Option<Level>>,
}

struct Logger(Arc<State>);

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("{} - {}", record.level(), record.args());
        //*self.0.last_log.lock().unwrap() = Some(record.level());
    }
    fn flush(&self) {}
}

pub fn init_log()
{
    let me = Arc::new(State { last_log: Mutex::new(None) });
    set_boxed_logger(Box::new(Logger(me))).unwrap();
    log::set_max_level(LevelFilter::Trace) ;
}
