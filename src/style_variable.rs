use iced::{Background, Color, Padding, Pixels};

pub struct StyleVariable {

  pub header_padding: Padding,

  pub header_spacing: f32,

  pub working_area_table_view_head_padding: Padding,

  pub working_area_table_view_head_spacing: Pixels,

  pub working_area_table_view_head_background: Background,

  pub working_area_table_view_body_padding: Padding,

  pub working_area_table_view_body_spacing: Pixels,

  pub working_area_table_view_body_operation_spacing: Pixels,

  pub working_area_table_view_body_background_0: Background,

  pub working_area_table_view_body_background_1: Background,

  pub working_area_table_view_body_link_cell_text_color: Color,

  pub status_bar_padding: Padding,

}

impl StyleVariable {

  pub fn new() -> Self {
    Self {
      header_padding: Padding {
        top: 6_f32,
        right: 12_f32,
        bottom: 6_f32,
        left: 12_f32,
      },
      header_spacing: 6_f32,
      working_area_table_view_head_padding: Padding {
        top: 6_f32,
        right: 12_f32,
        bottom: 6_f32,
        left: 12_f32,
      },
      working_area_table_view_head_spacing: Pixels(12_f32),
      working_area_table_view_head_background: Background::Color(
        Color::from_rgb8(
          63_u8,
          63_u8,
          63_u8,
        )
      ),
      working_area_table_view_body_padding: Padding {
        top: 6_f32,
        right: 12_f32,
        bottom: 6_f32,
        left: 12_f32,
      },
      working_area_table_view_body_spacing: Pixels(12_f32),
      working_area_table_view_body_operation_spacing: Pixels(6_f32),
      working_area_table_view_body_background_0: Background::Color(
        Color::from_rgb8(
          73_u8,
          73_u8,
          73_u8,
        )
      ),
      working_area_table_view_body_background_1: Background::Color(
        Color::from_rgb8(
          83_u8,
          83_u8,
          83_u8,
        )
      ),
      working_area_table_view_body_link_cell_text_color: Color::from_rgb8(
        0_u8,
        0_u8,
        255_u8,
      ),
      status_bar_padding: Padding {
        top: 6_f32,
        right: 12_f32,
        bottom: 6_f32,
        left: 12_f32,
      },
    }
  }
}
