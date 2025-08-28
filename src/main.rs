use iced::widget::{Column, Row, text};
use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Capitanus::run(Settings::default())
}

struct Capitanus {
    scripts: Vec<Script>,
}

#[derive(Debug, Clone)]
enum ScriptStatus {
    Idle,
    Running,
    Success,
    Fail,
    Inactive,
}

#[derive(Debug, Clone)]
struct Script {
    name: String,
    status: ScriptStatus,
}

impl Sandbox for Capitanus {
    type Message = ();

    fn new() -> Self {
        Self {
            scripts: vec![
                Script {
                    name: String::from("Dummy Script 1"),
                    status: ScriptStatus::Idle,
                },
                Script {
                    name: String::from("Dummy Script 2"),
                    status: ScriptStatus::Running,
                },
                Script {
                    name: String::from("Dummy Script 3"),
                    status: ScriptStatus::Success,
                },
            ],
        }
    }

    fn title(&self) -> String {
        String::from("Capitanus")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&self) -> Element<Self::Message> {
        const ITEMS_PER_ROW: usize = 4;
        let mut content = Column::new().spacing(20);

        for script_chunk in self.scripts.chunks(ITEMS_PER_ROW) {
            let mut row = Row::new().spacing(20);

            for script in script_chunk {
                row = row.push(text(&script.name))
            }
            content = content.push(row);
        }

        content.into()
    }
}
