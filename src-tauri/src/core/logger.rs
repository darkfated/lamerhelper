use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Success,
}

#[derive(Serialize, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub detail: Option<String>,
    pub indent: u8,
}

#[derive(Serialize)]
pub struct RunResult {
    pub ok: bool,
    pub message: String,
    pub logs: Vec<LogEntry>,
}

pub struct Logger {
    logs: Vec<LogEntry>,
    indent: u8,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            indent: 0,
        }
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.push(LogLevel::Info, message, None);
    }

    pub fn warn(&mut self, message: impl Into<String>) {
        self.push(LogLevel::Warn, message, None);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.push(LogLevel::Error, message, None);
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.push(LogLevel::Success, message, None);
    }

    pub fn section(&mut self, title: impl Into<String>) {
        self.push(LogLevel::Info, title, None);
    }

    pub fn kv(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.push(LogLevel::Info, label, Some(value.into()));
    }

    pub fn indent(&mut self) {
        self.indent = self.indent.saturating_add(1);
    }

    pub fn outdent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    pub fn with_indent<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Logger),
    {
        self.indent();
        f(self);
        self.outdent();
    }

    pub fn group<F>(&mut self, title: impl Into<String>, f: F)
    where
        F: FnOnce(&mut Logger),
    {
        self.section(title);
        self.with_indent(f);
    }

    fn push(&mut self, level: LogLevel, message: impl Into<String>, detail: Option<String>) {
        self.logs.push(LogEntry {
            level,
            message: message.into(),
            detail,
            indent: self.indent,
        });
    }

    pub fn into_logs(self) -> Vec<LogEntry> {
        self.logs
    }
}
