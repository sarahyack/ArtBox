use crate::ui::views::tool_view::ToolView;
use crate::ui::panels::main_view::MainViewTrait;
use crate::ui::panels::title_bar::TitleBarTrait;
use crate::ui::panels::info_panel::InfoPanelTrait;
<<<<<<< HEAD
use crate::ui::panels::footer::FooterTrait;
=======
use crate::ui::panels::footer::Footer;
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3

pub struct SpriteLibraryView {
    tool_name: String,
    sprite_count: usize,
    pub selected_sprite: Option<String>,
}

impl SpriteLibraryView {
    pub fn new() -> Self {
        Self {
<<<<<<< HEAD
            tool_name: String::from("Sprite Library"),
=======
            tool_name: "Sprite Library".to_string(),
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3
            sprite_count: 10,
            selected_sprite: None
        }
    }
}

impl ToolView for SpriteLibraryView {}

impl MainViewTrait for SpriteLibraryView {
   fn get_item_count(&self) -> usize {
      self.sprite_count
   }

   fn get_item_label(&self, index: usize) -> String {
      format!("Sprite {}", index + 1)
   }

   fn on_item_selected(&mut self, index: usize) {
      self.selected_sprite = Some(format!("Sprite {}", index + 1));
   }

   fn on_item_clicked(&mut self, index: usize) {
      self.selected_sprite = Some(format!("Sprite {}", index + 1));
   }

   // fn show_main_view(&mut self, ui: &mut egui::Ui) {
   //    self.show(ui);
   // }
}

impl TitleBarTrait for SpriteLibraryView {
<<<<<<< HEAD
    fn get_tool_title(&self) -> &String {
        &self.tool_name
=======
    fn get_tool_title(&self) -> String {
        self.tool_name
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3
    }

    fn on_tool_selected(&mut self) {
        // TODO
    }

    fn on_settings_clicked(&mut self) {
        // TODO
    }

    fn on_icon_clicked(&mut self) {
        // TODO
    }
}

<<<<<<< HEAD
// TODO: Move Item declaration to the utils module

pub struct Item {
=======
struct Item {
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3
    name: String,
    created_date: String,
    updated_date: String,
    tags: Vec<String>,
}

impl Item {
    pub fn new() -> Self {
        Item {
            name: String::from("Lorem Ipsum"),
            created_date: String::from("09-99-9999"),
            updated_date: String::from("99-99-9999"),
            tags: vec![String::from("Dorem"), String::from("Dolem")]
        }
    }
}

impl InfoPanelTrait for SpriteLibraryView {
    fn get_item(&self) -> Item {
        //TODO Add data fetching for selected sprite
<<<<<<< HEAD
        Item::new()
=======
        return Item::new()
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3
    }
    fn show_info_panel(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let (response, painter) = ui.allocate_painter(egui::vec2(100.0, 100.0), egui::Sense::hover());
            let preview_rect = response.rect;
            painter.rect_filled(preview_rect, egui::Rounding::same(5.0), egui::Color32::from_rgb(200, 200, 200));
            ui.add_space(10.0);
            ui.label("Sprite Preview");

            ui.add_space(10.0);
            let (response, painter) = ui.allocate_painter(egui::vec2(100.0, 100.0), egui::Sense::hover());
            let palette_rect = response.rect;
            let center = palette_rect.center();
            painter.circle_filled(center, 50.0, egui::Color32::from_rgb(150, 100, 200));
            ui.add_space(10.0);
            ui.label("Color Wheel");

            ui.add_space(10.0);
            ui.group(|ui| {
<<<<<<< HEAD
                let item = self.get_item();
                ui.label(format!("Name: {}", item.name));
                ui.label(format!("Created: {}", item.created_date));
                ui.label(format!("Updated: {}", item.updated_date));
                ui.label(format!("Tags: {}", item.tags.join(", ")));
=======
                ui.label(format!("Name: {}", self.name));
                ui.label(format!("Created: {}", self.created_date));
                ui.label(format!("Updated: {}", self.updated_date));
                ui.label(format!("Tags: {}", self.tags.join(", ")));
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3
            });
        });}
}

impl FooterTrait for SpriteLibraryView {
    fn show_footer(&self, ui: &mut egui::Ui) {
        ui.label("Footer");
    }
}