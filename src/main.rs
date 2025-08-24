use eframe::{Result, NativeOptions, App};
use egui::{ViewportBuilder};

use YOLO_Terminal::DB::connect;

struct WindowSize {
  width: f32,
  height: f32,
}

fn main() -> Result {
  env_logger::init();

  let _ = connect::db_connect();

  let default_window_size = WindowSize {
    width: 320.0,
    height: 240.0,
  };

  let window_width = default_window_size.width;
  let window_height = default_window_size.height;

  let options = NativeOptions {
    viewport: ViewportBuilder::default().with_inner_size([window_width, window_height]),
    ..Default::default()
  };


  let app_name = "Test GUI App";

  eframe::run_native(
    app_name,
    options,
    Box::new(|cc| {
      egui_extras::install_image_loaders(&cc.egui_ctx);
      Ok(Box::<AppQueryType>::default())
    }),
  )
}


struct AppQueryType {
  name: String,
  age: u32,
}

impl Default for AppQueryType {
  fn default() -> Self {
    Self {
      name: "".to_owned(),
      age: 0,
    }
  }
}

struct RangeSize {
  min: u32,
  max: u32,
}

impl App for AppQueryType {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let range_size = RangeSize {
      min: 0,
      max: 100,
    };
    
    let min_range = range_size.min;
    let max_range = range_size.max;

    egui::CentralPanel::default().show(ctx, |ui | {
      ui.heading("Test Application");
      ui.horizontal(|ui| {
        let name_label = ui.label("Name :");
        ui.text_edit_singleline(&mut self.name).labelled_by(name_label.id)
      });

      ui.add(egui::Slider::new(&mut self.age, min_range..=max_range).text("age"));
      if ui.button("Increment").clicked() {
        self.age += 1;
      }

      if ui.button("Decrement").clicked() {
        self.age -= 1;
      }

      ui.label(format!("Name: {}, Age: {}", self.name, self.age));
    });
  }
}