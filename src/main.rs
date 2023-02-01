use iced::widget::canvas::{Cache, Cursor, Geometry, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{alignment, executor, window, Event, Size};
use iced::{keyboard, Settings};
use iced::{Application, Color, Command, Element, Length, Point, Rectangle, Subscription, Theme};
use rand::prelude::*;

pub fn main() -> iced::Result {
    let width = 800_u32;
    let height = 600_u32;
    let pixel_size = 12_u32;

    DoomFire::run(Settings {
        flags: DoomFire::new(width, height, pixel_size),
        window: iced::window::Settings {
            size: (width, height),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    EventOccurred(Event),
}

#[derive(Debug)]
pub struct DoomFire {
    pub state: Cache,
    pub fire: Vec<u8>,
    pub pixel_size: u32,
    pub size: (u32, u32),
    pub color_palettes: Vec<Color>,
    debug: bool,
}

impl DoomFire {
    pub fn new(width: u32, height: u32, pixel_size: u32) -> DoomFire {
        let pixel_count = ((width / pixel_size) + 1) * ((height / pixel_size) + 1);
        let fire = vec![0; pixel_count as usize];

        DoomFire {
            fire,
            pixel_size,
            size: (width, height),
            ..Default::default()
        }
    }

    fn resize_fire(&mut self) {
        let cols = (self.size.0 / self.pixel_size) + 1;
        let rows = (self.size.1 / self.pixel_size) + 1;
        let total_pixels: u32 = cols * rows;
        self.fire.resize(total_pixels as usize, 0);
    }

    fn generate_fire_source(&mut self) {
        let cols = (self.size.0 / self.pixel_size) + 1;
        let rows = (self.size.1 / self.pixel_size) + 1;

        let total_pixels = cols * rows;
        let mut fire = vec![0; total_pixels as usize];

        for col in 0..cols {
            let pixel_index = (total_pixels - cols) + col;
            fire[pixel_index as usize] = 36;
        }

        self.fire = fire;
    }

    fn calculate_fire_propagation(&mut self) {
        let cols = (self.size.0 / self.pixel_size) + 1;
        let rows = (self.size.1 / self.pixel_size) + 1;

        for row in 0..rows {
            for col in 0..cols {
                let pixel_index = col + (cols * row);
                let below_pixel_index = pixel_index + cols;
                if below_pixel_index >= cols * rows {
                    continue;
                }

                let mut rng = rand::thread_rng();
                let decay: u32 = rng.gen_range(0..3);

                let below_pixel_intensity = self.fire[below_pixel_index as usize];

                // need to find a better solution to this conversions
                let temp_new_intensity = below_pixel_intensity as i32 - decay as i32;

                let new_pixel_intensity = if temp_new_intensity >= 0 {
                    temp_new_intensity as u32
                } else {
                    0
                };

                let wind_index = {
                    if (pixel_index as i32 - decay as i32) < 0 {
                        pixel_index
                    } else {
                        pixel_index - decay
                    }
                };

                self.fire[wind_index as usize] = new_pixel_intensity as u8;
            }
        }
    }

    fn get_color(&self, index: usize) -> Color {
        let pixel_value = self.fire[index];
        self.color_palettes[pixel_value as usize]
    }
}

impl Default for DoomFire {
    fn default() -> Self {
        DoomFire {
            fire: Vec::new(),
            pixel_size: 0,
            state: Cache::new(),
            size: (0, 0),
            debug: false,
            color_palettes: vec![
                Color::from_rgb8(7, 7, 7),
                Color::from_rgb8(31, 7, 7),
                Color::from_rgb8(47, 15, 7),
                Color::from_rgb8(71, 15, 7),
                Color::from_rgb8(87, 23, 7),
                Color::from_rgb8(103, 31, 7),
                Color::from_rgb8(119, 31, 7),
                Color::from_rgb8(143, 39, 7),
                Color::from_rgb8(159, 47, 7),
                Color::from_rgb8(175, 63, 7),
                Color::from_rgb8(191, 71, 7),
                Color::from_rgb8(199, 71, 7),
                Color::from_rgb8(223, 79, 7),
                Color::from_rgb8(223, 87, 7),
                Color::from_rgb8(223, 87, 7),
                Color::from_rgb8(215, 95, 7),
                Color::from_rgb8(215, 95, 7),
                Color::from_rgb8(215, 103, 15),
                Color::from_rgb8(207, 111, 15),
                Color::from_rgb8(207, 119, 15),
                Color::from_rgb8(207, 127, 15),
                Color::from_rgb8(207, 135, 23),
                Color::from_rgb8(207, 135, 23),
                Color::from_rgb8(199, 135, 23),
                Color::from_rgb8(199, 143, 23),
                Color::from_rgb8(199, 151, 31),
                Color::from_rgb8(191, 159, 31),
                Color::from_rgb8(191, 159, 31),
                Color::from_rgb8(191, 167, 39),
                Color::from_rgb8(191, 167, 39),
                Color::from_rgb8(191, 175, 47),
                Color::from_rgb8(183, 175, 47),
                Color::from_rgb8(183, 183, 47),
                Color::from_rgb8(183, 183, 55),
                Color::from_rgb8(207, 207, 111),
                Color::from_rgb8(223, 223, 159),
                Color::from_rgb8(239, 239, 199),
                Color::from_rgb8(255, 255, 255),
            ],
        }
    }
}

impl Application for DoomFire {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = DoomFire;

    fn new(state: Self::Flags) -> (Self, Command<Message>) {
        (state, Command::none())
    }

    fn title(&self) -> String {
        String::from("DoomFire - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                self.calculate_fire_propagation();
                self.state.clear();
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::Resized { width, height }) = event {
                    self.size = (width, height);
                    self.resize_fire();
                    self.generate_fire_source();
                    self.calculate_fire_propagation();
                    self.state.clear();
                }

                if let Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers: _modifiers,
                }) = event
                {
                    match key_code {
                        keyboard::KeyCode::D => {
                            self.debug = !self.debug;
                            self.state.clear();
                        }
                        keyboard::KeyCode::Up => {
                            if self.pixel_size < self.size.0 && self.pixel_size < self.size.1 {
                                self.pixel_size += 1;
                                self.resize_fire();
                                self.generate_fire_source();
                                self.calculate_fire_propagation();
                                self.state.clear();
                            }
                        }
                        keyboard::KeyCode::Down => {
                            if self.pixel_size > 1 {
                                self.pixel_size -= 1;
                                self.resize_fire();
                                self.generate_fire_source();
                                self.calculate_fire_propagation();
                                self.state.clear();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let canvas = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let ticks = iced::time::every(std::time::Duration::from_millis(50)).map(|_| Message::Tick);
        let events = iced_native::subscription::events().map(Message::EventOccurred);

        Subscription::batch(vec![ticks, events])
    }
}

impl<Message> canvas::Program<Message> for DoomFire {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let fire = self.state.draw(bounds.size(), |frame| {
            // Background Color
            frame.fill_rectangle(Point { x: 0.0, y: 0.0 }, bounds.size(), Color::BLACK);

            let rows = (self.size.1 / self.pixel_size) + 1;
            let columns = (self.size.0 / self.pixel_size) + 1;
            let pixel_size_widget = Size::new(self.pixel_size as f32, self.pixel_size as f32);

            for row in 0..rows {
                for column in 0..columns {
                    let pixel_index = column + (columns * row);

                    let position = Point {
                        x: (column * self.pixel_size) as f32,
                        y: (row * self.pixel_size) as f32,
                    };

                    let color = self.get_color(pixel_index as usize);

                    frame.fill_rectangle(position, pixel_size_widget, color);

                    if self.debug {
                        let debug_color = color.inverse();

                        frame.stroke(
                            &Path::rectangle(position, pixel_size_widget),
                            Stroke::default().with_color(debug_color),
                        );

                        let text_position = Point {
                            x: position.x + (self.pixel_size as f32 * 0.5),
                            y: position.y + (self.pixel_size as f32 * 0.5),
                        };

                        frame.fill_text(canvas::Text {
                            content: self.fire[pixel_index as usize].to_string(),
                            color: debug_color,
                            position: text_position,
                            horizontal_alignment: alignment::Horizontal::Center,
                            vertical_alignment: alignment::Vertical::Center,
                            size: self.pixel_size as f32 * 0.7,
                            ..Default::default()
                        });
                    }
                }
            }
        });

        vec![fire]
    }
}
