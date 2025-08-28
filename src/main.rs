use iced::widget::{Column, text};
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
        let scripts_list = self
            .scripts
            .iter()
            // Start with an empty column and add scripts to it
            .fold(Column::new(), |column, script| {
                column.push(text(&script.name))
            });
        scripts_list.into()
    }
}
