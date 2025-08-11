use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: usize,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: 0,
            max,
        }
    }

    pub fn set_value<T>(&self, value: &Rc<T>) {
        let count = Rc::strong_count(value);

        if self.max < count {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
        } else {
            let percentage = (count * 100) / self.max;
            if percentage > 70 {
                self.messages.borrow_mut().push(format!("Warning: You have used up over {}% of your quota!", percentage));
            }
        }
    }

    pub fn peek<T>(&self, value: &Rc<T>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percentage
        ));
    }
}