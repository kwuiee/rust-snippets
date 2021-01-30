use std::thread;
use std::time::Duration;

use delay_timer::prelude::*;

fn main() {
    let delay_timer = DelayTimerBuilder::default().build();

    // Add an asynchronous task to delay_timer.
    delay_timer
        .add_task(build_task(TaskBuilder::default()))
        .unwrap();
    // Since the tasks are executed in 8-second cycles,
    // we deal with something else.
    // Do someting about 8s.
    thread::sleep(Duration::new(60, 1_000_000));
    delay_timer.remove_task(1).unwrap();
    delay_timer.stop_delay_timer().unwrap();
}

fn build_task(mut task_builder: TaskBuilder) -> Task {
    let body = create_async_fn_body!({
        let mut res = surf::get("https://httpbin.org/get").await.unwrap();
        dbg!(res.body_string().await.unwrap());
    });

    task_builder
        .set_frequency_by_candy(CandyFrequency::Repeated(AuspiciousTime::PerEightSeconds))
        .set_task_id(2)
        .set_maximum_running_time(5)
        .spawn(body)
        .unwrap()
}

enum AuspiciousTime {
    PerSevenSeconds,
    PerEightSeconds,
    LoveTime,
}

impl Into<CandyCronStr> for AuspiciousTime {
    fn into(self) -> CandyCronStr {
        match self {
            Self::PerSevenSeconds => CandyCronStr("0/7 * * * * * *"),
            Self::PerEightSeconds => CandyCronStr("0/8 * * * * * *"),
            Self::LoveTime => CandyCronStr("0,10,15,25,50 0/1 * * Jan-Dec * 2020-2100"),
        }
    }
}
