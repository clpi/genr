use serde::{Deserialize, Serialize};
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput, ProgressBar, pane_grid, keyboard,
};

pub fn main() {
    App::run(Settings::default());
}

#[derive(Debug, Clone)]
pub enum Message {
    Saved(Result<(), String>),
    Loaded(Result<(), String>),
    InputChanged(String),
    Parse,
    Run,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    input: text_input::State,
    scroll: scrollable::State,
    controls: Controls,
    input_val: String,
}

#[derive(Debug, Default, Clone)]
pub struct Controls { 
    run_btn: button::State,
    parse_btn: button::State,
}

#[derive(Debug, Default)]
pub struct App {
    state: State,
    //theme: iced::
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self::default(), Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Genr") 
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let text_input = TextInput::new(
            &mut self.state.input,
            "Type something...",
            &self.state.input_val,
            Message::InputChanged,
        )
            .padding(10)
            .size(20);
        let mut parse_btn = Button::new(&mut self.state.controls.parse_btn, Text::new("Parse"))
            .padding(10)
            .on_press(Message::Parse);
        let mut run_btn = Button::new(&mut self.state.controls.run_btn, Text::new("Run"))
            .padding(10)
            .on_press(Message::Run);
        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(Row::new().spacing(10).push(text_input))
            .push(Row::new().spacing(10)
                .align_items(Align::Center)
                .push(parse_btn).push(run_btn));
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Parse => { Command::none() },
            _ => { Command::none() }
        }    
    }
}

fn hotkeys(event: pane_grid::KeyPressEvent) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::{Axis, Direction};

    let direction = match event.key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    };

    match event.key_code {
        _ => Some(Message::Parse),
    }
}
