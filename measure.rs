use std::env;
use std::process::{Command, Stdio};
use std::io::{self, Read};
use std::thread;
use std::time::{Duration, SystemTime};

fn main() -> io::Result<()> {
    const INTERVAL: u64 = 1;
    const ITERATE: usize = 5;

    let args = env::args().collect::<Vec<_>>();
    let (name, prog) = match args.as_slice() {
        [_, name, prog] => (name, prog),
        _ => panic!(),
    };

    for i in 0..ITERATE {
        let mut proc = Command::new(&prog)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let mut stdout = proc.stdout.take().unwrap();

        let t = thread::spawn::<_, io::Result<_>>(move || {
            thread::sleep(Duration::from_secs(INTERVAL));
            proc.kill()?;
            proc.wait()?;
            Ok(())
        });

        let begin = SystemTime::now();
        let mut buf = vec![0; 1024 * 8];
        let mut nbytes = 0u128;
        let mut lastrecv;
        loop {
            let n = stdout.read(&mut buf)?;
            nbytes += n as u128;
            lastrecv = SystemTime::now();
            if n == 0 {
                break
            }
        }
        t.join().unwrap()?;

        let duration = lastrecv.duration_since(begin).unwrap().as_millis();
        println!("{}\t{}\t{}", name, i, nbytes / duration);
    }
    Ok(())
}
