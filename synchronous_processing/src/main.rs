use std::sync::{Arc, Mutex, Condvar, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::env;

// 3.8.1 mutex
fn some_func(lock: Arc<Mutex<u64>>, max: i32, name: &'static str) {
    for _ in 1..max + 1 {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{} increment: {}", name, *val);
    }
}

// 3.8.2 condition
fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_all();
    println!("parent");
}

pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }

    pub fn wait(&self) {
        let mut cnt = self.mutex.lock().unwrap();
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap();
        }
        *cnt += 1;
    }

    pub fn post(&self) {
        let mut cnt = self.mutex.lock().unwrap();
        *cnt -= 1;
        if *cnt <= self.max {
            self.cond.notify_one();
        }
    }
}

fn mutex_example() {
    // 3.8.1 mutex
    // arcはスレッドセーフな参照カウンタ型のスマートポインタ
    let lock0 = Arc::new(Mutex::new(0));

    let lock1 =lock0.clone();

    let th0 = thread::spawn(move || {
        some_func(lock0, 10, "hoge");
    });

    let th1 = thread::spawn(move || {
        some_func(lock1, 20, "fuga");
    });

    th0.join().unwrap();
    th1.join().unwrap();
}

fn condition_example() {
    // 3.8.2 condition
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || { child(0, pair0)} );
    let c1 = thread::spawn(move || { child(1, pair1)} );
    let p = thread::spawn(move || { parent(pair2)} );

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}

fn barrier_example() {
    // 3.8.4 barrier
    let mut v = Vec::new();

    let barrier = Arc::new(Barrier::new(10));

    for i in 0..10 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            b.wait();
            println!("finished barrier {}", i);
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }
}

fn semaphore_example() {
    const NUM_LOOP: usize = 10;
    const NUM_THREAD: usize = 8;
    const SEM_NUM: isize = 4;

    static mut CNT: AtomicUsize = AtomicUsize::new(0);

    let mut v = Vec::new();
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for i in 0..NUM_THREAD {
        let s = sem.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                s.wait();

                unsafe { CNT.fetch_add(1, Ordering::SeqCst) };
                let n = unsafe { CNT.load(Ordering::SeqCst) };
                println!("semaphore: i = {}, CNT = {}", i, n);
                assert!((n as isize) <= SEM_NUM);
                unsafe { CNT.fetch_sub(1, Ordering::SeqCst) };
                
                s.post();
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}

fn help() {
    println!("Requires one argument. (mutex|condition|barrier)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "mutex" => mutex_example(),
                "condition" => condition_example(),
                "barrier" => barrier_example(),
                "semaphore" => semaphore_example(),
                _ => help()
            }
        },
        _ => help(),
    }

}
