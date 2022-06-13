pub struct TaskEntity {
    is_compleat: bool,
    tittle: String,
}

impl TaskEntity {
    pub fn new(tittle: &str) -> Self {
        Self { tittle: String::from(tittle), is_compleat: false }
    }

    pub fn is_compleat(&self) -> bool {
        self.is_compleat
    }

    pub fn compleat(&mut self) {
        self.is_compleat = true;
    }

    pub fn un_compleat(&mut self) {
        self.is_compleat = false;
    }

    pub fn to_string(&self) -> String {
        match self.is_compleat {
            true => format!("[x] {}", self.tittle).to_string(),
            false => format!("[ ] {}", self.tittle).to_string(),
        }
    }
}