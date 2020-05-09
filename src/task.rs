use crate::config::Options;
use std::{thread, time::Duration, time::Instant};

pub struct DomainUpdate {
    options: Options,
}

impl DomainUpdate {
    pub fn new(options: Options) -> Self {
        DomainUpdate { options: options }
    }

    pub fn run(&self) {
        let options = &self.options;
        let period_duration = Duration::from_secs(options.period as u64);

        // 循环执行任务
        loop {
            let start = Instant::now();
            log::info!("{:?}", options);

            // 防止因执行时间过长导致调度延误
            let used = Instant::now() - start;
            if used < period_duration {
                thread::sleep(period_duration - used);
            }
        }
    }
}
