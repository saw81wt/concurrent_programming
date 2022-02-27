use std::sync::{Arc, Mutex};
use std::thread;

struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],
    allocation: [[usize; NRES]; NTH],
    max: [[usize; NRES]; NTH],
}

impl <const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            let mut found = false;
            let mut num_true = 0;
            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                let need = self.max[i]
                    .iter()
                    .zip(alc)
                    .map(|(m, a)| m - a);
                let is_avail = work
                    .iter()
                    .zip(need)
                    .all(|(w, n)| *w >= n);
                
                if is_avail {
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a;
                    }
                    break;
                }
            }
            
            if num_true == NTH {
                return true;
            }

            if !found {
                break;
            }
        }

        false
    }

    fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH || resource >= NRES || self.available[resource] == 0 ||
            self.max[id][resource] == self.allocation[id][resource] {
                return false;
        }
        
        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true
        } else {
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl <const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(available, max)))
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource)
    }
}

pub fn bancker_example() {
    const NUM_LOOP: usize = 10;

    let banker = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);
    let banker0 = banker.clone();

    let philosopher0 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker0.take(0, 0) {}
            while !banker0.take(0, 1) {}

            println!("0: eating");

            banker0.release(0, 0);
            banker0.release(0, 1);
        }
    });
    
    let philosopher1 = thread::spawn(move || {
        for _ in 0..NUM_LOOP {
            while !banker.take(1, 1) {}
            while !banker.take(1, 0) {}

            println!("1: eating");

            banker.release(1, 1);
            banker.release(1, 0);
        }
    });

    philosopher0.join().unwrap();
    philosopher1.join().unwrap();
}
