use std::process::Command;
use std::{str, thread};
use std::str::FromStr;
use cron::Schedule;
use log::{debug, error, info};
use crate::config::ProcessGroup;
use chrono::{Utc};

pub(crate) fn run_process_group(process_group: Vec<ProcessGroup>){
    for process in process_group {
        thread::spawn(move || {
            let command = process.stream.clone();
            let thread_id = thread::current().id();
            debug!("{:?} thread id : {:?}", process.name, thread_id);
            let expression = process.cron.clone();
            match expression.as_str() {
                "now" => { execute_command(&command).expect("thread failed to execute command");},
                _ => {
                    debug!("Running cron expression: {}", expression);
                    execute_command_with_cron(&command, &expression);
                }
            }
        });
    }
}

/// Executes a command based on a given cron expression.
///
/// This function continuously checks the upcoming datetime based on the provided cron expression.
/// When the current datetime matches the cron expression, it executes the given command.
///
/// # Parameters
///
/// * `command` - A reference to a string representing the command to be executed.
/// * `expression` - A reference to a string representing the cron expression.
///
/// # Return
///
/// This function does not return any value. It continuously executes the command based on the cron expression.
pub(crate) fn execute_command_with_cron(command: &str, expression: &str){
    let schedule = Schedule::from_str(expression).unwrap();

    loop {
        let now = Utc::now();
        if let Some(datetime) = schedule.upcoming(Utc).take(1).next() {
            let until_next = datetime - now;
            thread::sleep(until_next.to_std().unwrap());
            info!("{} {} Running :  {}", datetime, expression, command);
            match execute_command(&command){
                Ok(_) => {},
                Err(err) => {
                    error!("thread failed to execute cron command: {}", err);
                }
            }
        }
    }
}

pub(crate) fn execute_command(command: &str) -> Result<bool, String> {
    // 确保日志记录器已经初始化
    debug!("Executing command: {}", command);
    // 创建 Command 对象
    let mut child = Command::new(command)
        .spawn()
        .map_err(|err| err.to_string())?;
    // 等待命令执行完成
    let status = child.wait().map_err(|err| err.to_string())?;
    // 检查命令是否成功执行
    Ok(status.success())
}
