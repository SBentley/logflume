use chrono::Local;
/* asynchronous execution of a serialised closure */
use lockfree::channel::spsc::create;
use std::thread;

// struct for wrapping the closure so it can be serialised
struct RawFunc {
    data: Box<dyn Fn() + Send + 'static>,
}

impl RawFunc {
    fn new<T>(data: T) -> RawFunc
    where
        T: Fn() + Send + 'static,
    {
        return RawFunc {
            data: Box::new(data),
        };
    }
    fn invoke(self) {
        (self.data)()
    }
}

impl std::fmt::Debug for RawFunc {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

fn main() {
    // create async thread to execute logging closure
    let (mut sx, mut rx) = create::<RawFunc>(); // lock free channel
    let _guard = thread::spawn(move || {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_affinity::set_for_current(*core_ids.last().unwrap());

        loop {
            match rx.recv() {
                Ok(msg) => {
                    msg.invoke();
                }
                Err(e) => {
                    // panic!("well this is not good");
                }
            }
        }
    });

    // strategy thread
    let date = Local::now(); // the timestamp should be from the strategy thread
    sx.send(RawFunc::new(move || {
        println!(
            "ts: {} volume: {} price: {} flag: {}",
            date.format("%Y-%m-%d %H:%M:%S"),
            100.02,
            20000.0,
            true
        );
    }))
    .unwrap(); // should really handle errors
}
