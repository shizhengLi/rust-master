#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilter {
    All,
    Open,
    Done,
}

#[derive(Debug, Default)]
pub struct TaskStore {
    tasks: Vec<Task>,
    next_id: u64,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: &str) -> Task {
        let task = Task {
            id: self.next_id,
            title: title.trim().to_string(),
            done: false,
        };

        self.next_id += 1;
        self.tasks.push(task.clone());
        task
    }

    pub fn complete(&mut self, id: u64) -> Option<&Task> {
        let task = self.tasks.iter_mut().find(|task| task.id == id)?;
        task.done = true;
        Some(task)
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn filtered(&self, filter: TaskFilter) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| match filter {
                TaskFilter::All => true,
                TaskFilter::Open => !task.done,
                TaskFilter::Done => task.done,
            })
            .collect()
    }

    pub fn to_storage_text(&self) -> String {
        self.tasks
            .iter()
            .map(|task| {
                let done = if task.done { "1" } else { "0" };
                format!("{}\t{}\t{}", task.id, done, escape_title(&task.title))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn from_storage_text(text: &str) -> Result<Self, String> {
        let mut tasks = Vec::new();
        let mut max_id = 0;

        for (index, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let mut parts = line.splitn(3, '\t');
            let id = parts
                .next()
                .ok_or_else(|| format!("line {} is missing an id", index + 1))?
                .parse::<u64>()
                .map_err(|_| format!("line {} has an invalid id", index + 1))?;
            let done = match parts
                .next()
                .ok_or_else(|| format!("line {} is missing a status", index + 1))?
            {
                "0" => false,
                "1" => true,
                _ => return Err(format!("line {} has an invalid status", index + 1)),
            };
            let title = parts
                .next()
                .ok_or_else(|| format!("line {} is missing a title", index + 1))
                .map(unescape_title)?;

            max_id = max_id.max(id);
            tasks.push(Task { id, title, done });
        }

        Ok(Self {
            tasks,
            next_id: max_id + 1,
        })
    }
}

fn escape_title(title: &str) -> String {
    title
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape_title(title: &str) -> String {
    let mut output = String::new();
    let mut chars = title.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => output.push('\t'),
                Some('n') => output.push('\n'),
                Some('\\') => output.push('\\'),
                Some(other) => {
                    output.push('\\');
                    output.push(other);
                }
                None => output.push('\\'),
            }
        } else {
            output.push(ch);
        }
    }

    output
}
