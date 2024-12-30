use iced::widget::{button, column, container, row, text};
use iced::{alignment, Alignment, Element, Length, Task, Theme};

pub fn main() -> iced::Result {
    iced::application(PomodoroTimer::title, PomodoroTimer::update, PomodoroTimer::view)
        .subscription(PomodoroTimer::subscription)
        .theme(PomodoroTimer::theme)
        .run_with(PomodoroTimer::new)
}

struct PomodoroTimer {
    seconds_left: i32,
    is_running: bool,
    is_work: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    ToggleTimer,
    Reset,
    SwitchMode,
}

impl PomodoroTimer {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                seconds_left: 25 * 60, // 25 minutes for work session
                is_running: false,
                is_work: true,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("🧊🍅 - Pomodoro Timer")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                if self.is_running && self.seconds_left > 0 {
                    self.seconds_left -= 1;
                }
            }
            Message::ToggleTimer => {
                self.is_running = !self.is_running;
            }
            Message::Reset => {
                self.seconds_left = if self.is_work { 25 * 60 } else { 5 * 60 };
                self.is_running = false;
            }
            Message::SwitchMode => {
                self.is_work = !self.is_work;
                self.seconds_left = if self.is_work { 25 * 60 } else { 5 * 60 };
                self.is_running = false;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let minutes = self.seconds_left / 60;
        let seconds = self.seconds_left % 60;
        let time_text = format!("{:02}:{:02}", minutes, seconds);
        let mode_text = if self.is_work { "Work Time" } else { "Break Time" };

        let timer_button_text = if self.is_running { "Pause" } else { "Start" };

        let controls = row![
            button(timer_button_text).on_press(Message::ToggleTimer),
            button("Reset").on_press(Message::Reset),
            button("Switch Mode").on_press(Message::SwitchMode),
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        container(
            column![
                text(mode_text).size(40),
                text(time_text).size(60),
                controls
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen_raw(|event, _, _status| {
            match event {
                iced::Event::Window(iced::window::Event::RedrawRequested(_)) => {
                    Some(Message::Tick)
                }
                _ => None,
            }
        })
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}