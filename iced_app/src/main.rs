use iced::widget::{button, column, container, row, slider, text, Column, Row};
use iced::{Alignment, Color, Element, Length, Theme};

pub fn main() -> iced::Result {
    iced::application("Color Picker", ColorChooser::update, ColorChooser::view)
        .window_size([320.0, 480.0])
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Red(u8),
    Green(u8),
    Blue(u8),
    Save,
}

#[derive(Default)]
struct ColorChooser {
    red: u8,
    green: u8,
    blue: u8,
    #[allow(dead_code)]
    favorites: Vec<(u8, u8, u8)>,
}

impl ColorChooser {
    fn update(&mut self, message: Message) {
        match message {
            Message::Red(r) => {
                self.red = r;
            }
            Message::Green(g) => {
                self.green = g;
            }
            Message::Blue(b) => {
                self.blue = b;
            }
            Message::Save => {
                let color = (self.red, self.green, self.blue);
                if !self.favorites.contains(&color) {
                    self.favorites.push(color)
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                container("")
                    .width(Length::Fill)
                    .height(Length::Fixed(100.0))
                    .style(|_theme| {
                        container::Style {
                            background: Some(
                                Color::from_rgb8(
                                    self.red, self.green, self.blue,
                                )
                                .into(),
                            ),
                            ..Default::default()
                        }
                    }),
                color_slider("Red", self.red, Message::Red),
                color_slider("Green", self.green, Message::Green),
                color_slider("Blue", self.blue, Message::Blue),
                text(format!(
                    "Hex: #{:02X}{:02X}{:02X}",
                    self.red, self.green, self.blue
                )),
                button("Add to Favorites").on_press(Message::Save),
                container(render_favorites(&self.favorites))
                    .width(Length::Fill)
                    .align_x(Alignment::Start)
            ]
            .spacing(12),
        )
        .padding(20)
        .into()
    }
}

fn color_slider<'a>(
    label: &'a str,
    value: u8,
    on_change: impl Fn(u8) -> Message + 'a,
) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fixed(50.0)),
        slider(0..=255, value, on_change),
        text(value).width(Length::Fixed(30.0))
    ]
    .spacing(6)
    .align_y(Alignment::Center)
    .into()
}

fn render_favorites<'a>(colors: &'a [(u8, u8, u8)]) -> Element<'a, Message> {
    let items_per_row = 11;
    let mut column = Column::new().spacing(4);

    for chunk in colors.chunks(items_per_row) {
        let row = Row::with_children(
            chunk
                .iter()
                .map(|(r, g, b)| {
                    container("")
                        .width(Length::Fixed(20.0))
                        .height(Length::Fixed(20.0))
                        .style(|_: &Theme| container::Style {
                            background: Some(
                                Color::from_rgb8(*r, *g, *b).into(),
                            ),
                            ..Default::default()
                        })
                        .into()
                })
                .collect::<Vec<Element<_>>>(),
        )
        .spacing(4);

        column = column.push(row);
    }

    column.into()
}
