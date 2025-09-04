use iced::{widget::Text, Application, Font, Program};

pub fn load<P: Program>(app: Application<P>) -> Application<P> {
  app
    .font(include_bytes!("./assets/Material Icons.ttf"))
    .font(include_bytes!("./assets/Material Icons Two Tone.otf"))
    .font(include_bytes!("./assets/Material Icons Sharp.otf"))
    .font(include_bytes!("./assets/Material Icons Round.otf"))
    .font(include_bytes!("./assets/Material Icons Outlined.otf"))
}

pub fn keyboard_arrow_down_round() -> Text<'static> {
  Text::new("\u{e313}").font(Font::with_name("Material Icons Round"))
}

pub fn keyboard_arrow_right_round() -> Text<'static> {
  Text::new("\u{e315}").font(Font::with_name("Material Icons Round"))
}
