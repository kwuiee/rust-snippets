use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler, JobToRun};

#[tokio::main]
async fn main() {
    let mut sched = JobScheduler::new();

    sched.add(
        Job::new("1/10 * * * * *", |uuid, l| {
            println!("I run every 10 seconds");
        })
        .unwrap(),
    );

    sched.add(
        Job::new_async("1/7 * * * * *", |uuid, l| {
            Box::pin(async {
                println!("I run async every 7 seconds");
            })
        })
        .unwrap(),
    );

    sched.add(
        Job::new("1/30 * * * * *", |uuid, l| {
            println!("I run every 30 seconds");
        })
        .unwrap(),
    );

    sched.add(
        Job::new_one_shot(Duration::from_secs(18), |_uuid, _l| {
            println!("{:?} I'm only run once", chrono::Utc::now());
        })
        .unwrap(),
    );

    let mut jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
        println!("{:?} I'm repeated every 8 seconds", chrono::Utc::now());
    })
    .unwrap();

    jj.on_start_notification_add(Box::new(|job_id, notification_id, type_of_notification| {
        Box::pin(async move {
            println!(
                "Job {:?} was started, notification {:?} ran ({:?})",
                job_id, notification_id, type_of_notification
            );
        })
    }));

    jj.on_stop_notification_add(Box::new(|job_id, notification_id, type_of_notification| {
        Box::pin(async move {
            println!(
                "Job {:?} was completed, notification {:?} ran ({:?})",
                job_id, notification_id, type_of_notification
            );
        })
    }));

    jj.on_removed_notification_add(Box::new(|job_id, notification_id, type_of_notification| {
        Box::pin(async move {
            println!(
                "Job {:?} was removed, notification {:?} ran ({:?})",
                job_id, notification_id, type_of_notification
            );
        })
    }));
    sched.add(jj);

    let five_s_job = Job::new("1/5 * * * * *", |_uuid, _l| {
        println!("{:?} I run every 5 seconds", chrono::Utc::now());
    })
    .unwrap();
    sched.add(five_s_job);

    let four_s_job_async = Job::new_async("1/4 * * * * *", |_uuid, _l| {
        Box::pin(async move {
            println!("{:?} I run async every 4 seconds", chrono::Utc::now());
        })
    })
    .unwrap();
    sched.add(four_s_job_async);

    sched.add(
        Job::new("1/30 * * * * *", |_uuid, _l| {
            println!("{:?} I run every 30 seconds", chrono::Utc::now());
        })
        .unwrap(),
    );

    sched.add(
        Job::new_one_shot(Duration::from_secs(18), |_uuid, _l| {
            println!("{:?} I'm only run once", chrono::Utc::now());
        })
        .unwrap(),
    );

    sched.add(
        Job::new_one_shot_async(Duration::from_secs(16), |_uuid, _l| {
            Box::pin(async move {
                println!("{:?} I'm only run once async", chrono::Utc::now());
            })
        })
        .unwrap(),
    );

    let jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
        println!("{:?} I'm repeated every 8 seconds", chrono::Utc::now());
    })
    .unwrap();
    sched.add(jj);

    let jja = Job::new_repeated_async(Duration::from_secs(7), |_uuid, _l| {
        Box::pin(async move {
            println!(
                "{:?} I'm repeated async every 7 seconds",
                chrono::Utc::now()
            );
        })
    })
    .unwrap();
    sched.add(jja);

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    sched.start().await;
}
