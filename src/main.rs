use clap::{App, Arg};
use std::{
    error::Error,
    io::{self, Read},
    sync::{Arc, Condvar, Mutex},
    thread,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .about("A modern word counter written with performance in mind")
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("print the byte counts"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("print the newline counts"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("print the word counts"),
        )
        .arg(Arg::with_name("FILE").multiple(true))
        .get_matches();

    let chunk_size = 0x1000000usize;
    let final_count = Arc::new(Mutex::new(wcr::Count::new()));

    let max_threads = num_cpus::get();

    let pair = Arc::new((Mutex::new(0usize), Condvar::new()));
    let (lock, cvar) = &*pair;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = io::stdin()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)?;
        if n == 0 {
            break;
        }

        let fcount = final_count.clone();

        let mut thread_count = lock.lock().unwrap();
        while *thread_count >= max_threads {
            thread_count = cvar.wait(thread_count).unwrap();
        }

        *thread_count += 1;

        let thread_pair = pair.clone();
        thread::spawn(move || {
            let count = wcr::parse(chunk);
            let mut fc = fcount.lock().unwrap();
            *fc += count;
            let (lock, cvar) = &*thread_pair;

            let mut thread_count = lock.lock().unwrap();
            *thread_count -= 1;
            cvar.notify_one();
        });

        if n < chunk_size {
            break;
        }
    }

    let _guard = cvar
        .wait_while(lock.lock().unwrap(), |thread_count| thread_count > &mut 0)
        .unwrap();

    let mut count = final_count.lock().unwrap();

    if count.chars > 0 {
        count.lines += 1;
    }

    if matches.is_present("bytes") {
        println!("{}", count.chars);
    } else if matches.is_present("lines") {
        println!("{}", count.lines);
    } else if matches.is_present("words") {
        println!("{}", count.words);
    } else {
        println!("{:?}", count);
    }
    Ok(())
}
