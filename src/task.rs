use anyhow::Result;

pub struct Task {
    name: String,
    due: String,
    recur: String,
}

impl Task {
    pub fn from_string(string: &str) -> Result<Task> {
        if !string.starts_with("- [ ] ") {
            panic!("It's not a task!")
        }
        let string = &string[6..];
        let mut name = String::new();
        let mut due = String::new();
        let mut recur = String::new();
        let mut elements: Vec<&str> = string.split(" ").collect();
        'outer: while !elements.is_empty() {
            for field in ["due", "recur"] {
                if elements[0].starts_with(field) {
                    break 'outer;
                }
            }
            let element = elements.remove(0);
            name.push_str(format!(" {}", &element).as_str());
        }
        name = name.trim().to_string();
        for element in elements {
            if element.starts_with("due") {
                let split_result: Vec<&str> = element.split(":").collect();
                due = split_result[1].to_string();
            } else if element.starts_with("recur") {
                let split_result: Vec<&str> = element.split(":").collect();
                recur = split_result[1].to_string();
            }
        }
        Ok(Task {
            name: name,
            due: due,
            recur: recur,
        })
    }

    pub fn to_string(&self) -> String {
        format!("- [ ] {} due:{} recur:{}", self.name, self.due, self.recur)
    }
}
