#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let woker = Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(woker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let worker = self.grade.take()?;
        self.grade = worker.next;
        Some(worker.name)
    }

    pub fn last_worker(&mut self) -> Option<(String, Role)> {
        if let Some(worker) = &self.grade {
            return Some((worker.name.clone(), worker.role.clone()));
        }
        None
    }
}