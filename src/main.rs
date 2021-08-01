use std::{
    error::Error,
    io::{self, Read},
    sync::{Arc, Condvar, Mutex},
    thread,
};

fn main() -> Result<(), Box<dyn Error>> {
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
    }

    let _guard = cvar
        .wait_while(lock.lock().unwrap(), |thread_count| thread_count > &mut 0)
        .unwrap();

    let mut count = final_count.lock().unwrap();

    if count.chars > 0 {
        count.lines += 1;
    }

    println!("{:?}", count);
    Ok(())
}
