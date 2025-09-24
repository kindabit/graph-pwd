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

pub fn language_round() -> Text<'static> {
  Text::new("\u{e894}").font(Font::with_name("Material Icons Round"))
}

pub fn help_outline_round() -> Text<'static> {
  Text::new("\u{e8fd}").font(Font::with_name("Material Icons Round"))
}

pub fn east_round() -> Text<'static> {
  Text::new("\u{f1df}").font(Font::with_name("Material Icons Round"))
}

pub fn west_round() -> Text<'static> {
  Text::new("\u{f1e6}").font(Font::with_name("Material Icons Round"))
}

pub fn stop_circle_round_x6() -> Text<'static> {
  Text::new("\u{ef71}\u{ef71}\u{ef71}\u{ef71}\u{ef71}\u{ef71}").font(Font::with_name("Material Icons Round"))
}

pub fn remove_red_eye_round() -> Text<'static> {
  Text::new("\u{e417}").font(Font::with_name("Material Icons Round"))
}

pub fn content_copy_round() -> Text<'static> {
  Text::new("\u{f08a}").font(Font::with_name("Material Icons Round"))
}

pub fn home_round() -> Text<'static> {
  Text::new("\u{e88a}").font(Font::with_name("Material Icons Round"))
}

pub fn account_circle_round() -> Text<'static> {
  Text::new("\u{e853}").font(Font::with_name("Material Icons Round"))
}

pub fn lock_round() -> Text<'static> {
  Text::new("\u{e897}").font(Font::with_name("Material Icons Round"))
}

pub fn mouse_round() -> Text<'static> {
  Text::new("\u{e323}").font(Font::with_name("Material Icons Round"))
}

pub fn settings_round() -> Text<'static> {
  Text::new("\u{e8b8}").font(Font::with_name("Material Icons Round"))
}
