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

pub fn more_round() -> Text<'static> {
  Text::new("\u{e619}").font(Font::with_name("Material Icons Round"))
}

pub fn edit_round() -> Text<'static> {
  Text::new("\u{e3c9}").font(Font::with_name("Material Icons Round"))
}

pub fn delete_round() -> Text<'static> {
  Text::new("\u{e872}").font(Font::with_name("Material Icons Round"))
}

pub fn person_add_round() -> Text<'static> {
  Text::new("\u{e7fe}").font(Font::with_name("Material Icons Round"))
}
