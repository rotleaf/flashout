pub mod counter {
    use std::{
        error::Error,
        thread,
        time::Duration,
        {io::stdout, io::Write},
    };

    pub async fn init(mut minutes: u32) -> Result<(), Box<dyn Error>> {
        let mut seconds = 0;
        let mut stdout = stdout();

        loop {
            let time_str = format!("{:02}:{:02}", minutes, seconds);
            print!("\r please wait {} ...", time_str);
            stdout.flush().unwrap();

            if minutes == 0 && seconds == 0 {
                break;
                // times up
            }

            thread::sleep(Duration::from_secs(1));

            if seconds == 0 {
                if minutes > 0 {
                    minutes -= 1;
                    seconds = 59;
                }
            } else {
                seconds -= 1;
            }
        }

        Ok(())
    }
}
