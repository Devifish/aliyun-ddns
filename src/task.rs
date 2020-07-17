use crate::aliyun;
use crate::config::Options;
use crate::ip;
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
        if options.domains.len() == 0 {
            panic!("缺少域名信息");
        }

        let period_duration = Duration::from_secs(options.period as u64);
        // 循环执行任务
        loop {
            let start = Instant::now();
            let result = aliyun::split_records(options).and_then(|records| {
                aliyun::list_records(options, &records).and_then(|(update_rs, create_rs)| {
                    let _x = ip::get_ips().and_then(|ips| {
                        log::info!("本地公网IP信息:{:?}", ips);
                        if update_rs.len() > 0 {
                            aliyun::update_records(options, &ips, &update_rs)?;
                        }
                        if create_rs.len() > 0 {
                            aliyun::create_records(options, &ips, &create_rs)?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })
            });
            if let Err(e) = result {
                log::error!("阿里云端处理失败, error:{:?}", e);
            }

            // 防止因执行时间过长导致调度延误
            let used = Instant::now() - start;
            if used < period_duration {
                thread::sleep(period_duration - used);
            }
        }
    }
}
