// Import necessary modules and components from the iced crate
use iced::widget::{button, column, container, row, text};
use iced::{alignment, Alignment, Element, Length, Task, Theme};
use std::time::Instant;

// Main function to run the Pomodoro Timer application
pub fn main() -> iced::Result {
    iced::application(PomodoroTimer::title, PomodoroTimer::update, PomodoroTimer::view)
        .subscription(PomodoroTimer::subscription)
        .theme(PomodoroTimer::theme)
        .run_with(PomodoroTimer::new)
}

// Struct representing the state of the Pomodoro Timer
struct PomodoroTimer {
    seconds_left: i32,       // Seconds left in the current session
    is_running: bool,        // Whether the timer is currently running
    is_work: bool,           // Whether the current session is a work session
    last_tick: Option<Instant>, // The last time the timer was ticked
}

// Enum representing the different messages that can be sent to the Pomodoro Timer
#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,          // Message to indicate a tick of the timer
    ToggleTimer,   // Message to toggle the timer on/off
    Reset,         // Message to reset the timer
    SwitchMode,    // Message to switch between work and break modes
}

// Implementation of the PomodoroTimer struct
impl PomodoroTimer {


    // Function to create a new PomodoroTimer instance
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                // Initialize the PomodoroTimer with default values
                seconds_left: 1500, // Default to 25 minutes
                is_running: false,
                is_work: true,
                last_tick: None,
            },
            Task::none(), // No initial task
        )
    }



    // Function to return the title of the Pomodoro Timer application
    fn title(&self) -> String {
        String::from("🧊🍅 - Pomodoro Timer - 🍅🧊")
    }




    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                if self.is_running {
                    let now = Instant::now();
                    
                    if let Some(last_tick) = self.last_tick {
                        let duration = now.duration_since(last_tick);
                        
                        if duration.as_secs() >= 1 && self.seconds_left > 0 {
                            self.seconds_left -= 1;
                            self.last_tick = Some(now);
                        }
                    } else {
                        self.last_tick = Some(now);
                    }
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
        ]
        .spacing(10)
        .align_y(Alignment::Center);


        let buttons = column![
            controls,
            button("Switch Mode").on_press(Message::SwitchMode).style(button::success).width(Length::Fill),
        ]
        .spacing(20)
        .align_x(Alignment::Center);


        container(
            column![
                text(mode_text).size(40),
                text(time_text).size(60),
                buttons
            ]
            .spacing(20)
            .align_x(Alignment::Center)
            .width(200)
            .height(300)
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
        Theme::TokyoNightStorm
    }
}