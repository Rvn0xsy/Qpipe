use hello_rust::config::Config;


use std::io::Read;


#[test]
fn name() {
   assert_eq!(1, 1);
}
#[test]
pub fn hello(){
    println!("Hello, world!");
    assert_eq!(1,1);
}

enum AnyValue {
    Tuple(Vec<i32>), // 代表一个整数元组
    Array([i32; 5]), // 代表一个固定大小的整数数组
    Integer(i32),    // 代表一个整数
}

fn print_any_value(value: AnyValue) {
    match value {
        AnyValue::Tuple(ref tuple) => {
            println!("Tuple: {:?}", tuple);
            for i in tuple {
                println!("{}", i);
            }
        },
        AnyValue::Array(ref array) => {
            println!("Array: {:?}", array);
            for &i in array {
                println!("{}", i);
            }
        },
        AnyValue::Integer(integer) => {
            println!("Integer: {}", integer);
        },
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, Utc};
    use cron::Schedule;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::str::FromStr;
    use std::thread::sleep;
    use std::io;

    pub fn read_from_pipe(pipe_path: &str) -> io::Result<String> {
        // 打开命名管道
        let file = File::open(pipe_path)?;
        let mut reader = BufReader::new(file);
        // 创建一个String来存储读取的内容
        let mut content = String::new();
        // 循环读取数据直到到达流的末尾
        loop {
            // 读取数据到content中，返回读取的字节数
            let bytes_read = reader.read_to_string(&mut content)?;
            // 如果读取到的字节数为0，则表示到达流的末尾
            if bytes_read == 0 {
                break;
            }
            // 可以在这里处理读取到的数据，例如，将其追加到content中
            // content.push_str(&content); // 这将重复内容，通常不需要这样做
        }
        Ok(content)
    }
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_config() {
        
        // let config_path = env::current_dir().unwrap().join("tests/config.yaml");
        // let global_conf = Config::fetch_conf(Some(config_path));
        // println!("{:?}", global_conf);
        assert_eq!(1,1);
    }
    
    #[test]
    fn write_stdout(){
        let stdout = "file:///dev/stdout";
    }
    #[test]
    fn read_pipe() {
        // 命名管道的路径
        let pipe_path = "/tmp/input_pipe";
        // 调用函数并处理结果
        match read_from_pipe(pipe_path) {
            Ok(content) => {
                println!("从管道读取的内容:\n{}", content);
            }
            Err(e) => {
                eprintln!("读取管道时发生错误: {}", e);
            }
        }
    }
    
    #[test]
    fn test_cron() {
        let expression = "0/5 * * * * *";
        let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

        loop {
            let now = Utc::now();
            if let Some(next) = schedule.upcoming(Utc).take(1).next() {
                let until_next = next - now;
                sleep(until_next.to_std().unwrap());
                println!(
                    "Running every 5 seconds. Current time: {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S")
                );
            }
        }
    }
}
