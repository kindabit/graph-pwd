use std::sync::{Arc, Mutex, MutexGuard};

use iced::{Background, Color, Length, Padding, Pixels};

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

  pub working_area_table_view_body_deleted_background_0: Background,

  pub working_area_table_view_body_deleted_background_1: Background,

  pub working_area_table_view_body_link_cell_text_color: Color,

  pub working_area_table_view_scrollbar_width: Pixels,

  pub working_area_table_view_scroller_width: Pixels,

  pub working_area_table_view_scrollbar_margin: Pixels,

  pub working_area_table_view_footer_padding: Padding,

  pub working_area_table_view_footer_jump_to_input_width: Pixels,

  pub working_area_table_view_footer_spacing: Pixels,

  pub status_bar_padding: Padding,

  pub mini_account_selector_height: Length,

  pub mini_account_selector_scrollbar_width: Pixels,

  pub mini_account_selector_scroller_width: Pixels,

  pub mini_account_selector_scrollbar_margin: Pixels,

  pub mini_account_selector_table_account_id_width: Length,

  pub mini_account_selector_selected_account_background: Background,

  pub add_or_edit_account_dialog_form_padding: Padding,

  pub add_or_edit_account_dialog_scrollbar_width: Pixels,

  pub add_or_edit_account_dialog_scrollbar_margin: Pixels,

  pub add_or_edit_account_dialog_scroller_width: Pixels,

  pub account_detail_dialog_content_padding: Padding,

  pub account_detail_dialog_scrollbar_width: Pixels,

  pub account_detail_dialog_scrollbar_margin: Pixels,

  pub account_detail_dialog_scroller_width: Pixels,

  pub working_area_table_view_search_box_padding: Padding,

  pub working_area_table_view_search_box_middle_space_width: Length,

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
          53_u8,
          53_u8,
          53_u8,
        )
      ),
      working_area_table_view_body_padding: Padding {
        top: 4_f32,
        right: 12_f32,
        bottom: 4_f32,
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
          63_u8,
          63_u8,
          63_u8,
        )
      ),
      working_area_table_view_body_deleted_background_0: Background::Color(
        Color::from_rgb8(
          127_u8,
          0_u8,
          0_u8,
        )
      ),
      working_area_table_view_body_deleted_background_1: Background::Color(
        Color::from_rgb8(
          137_u8,
          0_u8,
          0_u8,
        )
      ),
      working_area_table_view_body_link_cell_text_color: Color::from_rgb8(
        0_u8,
        0_u8,
        255_u8,
      ),
      working_area_table_view_scrollbar_width: Pixels(6_f32),
      working_area_table_view_scroller_width: Pixels(6_f32),
      working_area_table_view_scrollbar_margin: Pixels(6_f32),
      working_area_table_view_footer_padding: Padding {
        top: 6_f32,
        right: 12_f32,
        bottom: 0_f32,
        left: 12_f32,
      },
      working_area_table_view_footer_jump_to_input_width: Pixels(96_f32),
      working_area_table_view_footer_spacing: Pixels(6_f32),
      status_bar_padding: Padding {
        top: 6_f32,
        right: 12_f32,
        bottom: 6_f32,
        left: 12_f32,
      },
      mini_account_selector_height: Length::Fixed(200_f32),
      mini_account_selector_scrollbar_width: Pixels(6_f32),
      mini_account_selector_scroller_width: Pixels(6_f32),
      mini_account_selector_scrollbar_margin: Pixels(6_f32),
      mini_account_selector_table_account_id_width: Length::Fixed(32_f32),
      mini_account_selector_selected_account_background: Background::Color(
        Color::from_rgb8(
          127_u8,
          127_u8,
          127_u8,
        )
      ),
      add_or_edit_account_dialog_form_padding: Padding {
        top: 6_f32,
        right: 18_f32,
        bottom: 6_f32,
        left: 6_f32,
      },
      add_or_edit_account_dialog_scrollbar_width: Pixels(6_f32),
      add_or_edit_account_dialog_scroller_width: Pixels(6_f32),
      add_or_edit_account_dialog_scrollbar_margin: Pixels(6_f32),
      account_detail_dialog_content_padding: Padding {
        top: 6_f32,
        right: 18_f32,
        bottom: 6_f32,
        left: 6_f32,
      },
      account_detail_dialog_scrollbar_width: Pixels(6_f32),
      account_detail_dialog_scroller_width: Pixels(6_f32),
      account_detail_dialog_scrollbar_margin: Pixels(6_f32),
      working_area_table_view_search_box_padding: Padding {
        top: 0_f32,
        right: 12_f32,
        bottom: 6_f32,
        left: 12_f32,
      },
      working_area_table_view_search_box_middle_space_width: Length::Fixed(6_f32),
    }
  }

  pub fn working_area_table_view_body_background(&self, row_index: usize, deleted: bool) -> Background {
    match deleted {
      true => {
        match row_index % 2 {
          0 => self.working_area_table_view_body_deleted_background_0,
          1.. => self.working_area_table_view_body_deleted_background_1,
        }
      }
      false => {
        match row_index % 2 {
          0 => self.working_area_table_view_body_background_0,
          1.. => self.working_area_table_view_body_background_1,
        }
      }
    }
  }

  pub fn lock(r: &Arc<Mutex<Self>>) -> MutexGuard<Self> {
    r.lock().expect("fail to lock StyleVariable")
  }

}
