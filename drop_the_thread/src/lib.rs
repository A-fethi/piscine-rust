use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let next_pid = self.thread_len();
        self.states.borrow_mut().push(false);
        (next_pid, Thread::new(next_pid, c, self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        let states = self.states.borrow();
        if id < states.len() {
            if states[id] == true {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    pub fn drop_thread(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] == true {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        let current_value = self.drops.get();
        let new_value = current_value + 1;
        self.drops.set(new_value);
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Self {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}