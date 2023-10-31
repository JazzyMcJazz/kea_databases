use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct App {
    pub is_rdb: bool,
    pub is_ddb: bool,
    pub is_gdb: bool,
}

impl App {
    pub fn from_path(path: &str) -> Self {
        let mut app = App {
            is_rdb: false,
            is_ddb: false,
            is_gdb: false,
        };

        if path.starts_with("/relania") {
            app.is_rdb = true;
        } else if path.starts_with("/documenia") {
            app.is_ddb = true;
        } else if path.starts_with("/graphia") {
            app.is_gdb = true;
        }

        app
    }
}
