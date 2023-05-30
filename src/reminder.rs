use chrono::{NaiveTime, Timelike, Datelike, Utc};
use job_scheduler::{Job, JobScheduler};

pub fn schedule_reminder(sched: &mut JobScheduler, task: String, reminder_time: NaiveTime) {
    sched.add(Job::new(
        format!(
            "{} {} {} {} {} *",
            reminder_time.second(),
            reminder_time.minute(),
            reminder_time.hour(),
            Utc::now().day(),
            Utc::now().month()
        )
        .parse()
        .unwrap(),
        move || {
            println!("Lembrete: sua tarefa '{}' está agendada para agora", task)
        },
    ));
}
