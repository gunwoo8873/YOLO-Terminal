use eframe::{egui, Result, NativeOptions};
use egui::{ViewportBuilder};

struct WindowSize {
  width: f32,
  height: f32,
}

fn main() -> Result {
  env_logger::init();

  let size = WindowSize {
    width: 320.0,
    height: 240.0,
  };

  let window_width = size.width;
  let window_height = size.height;

  let options = NativeOptions {
    viewport: ViewportBuilder::default().with_inner_size([window_width, window_height]),
    ..Default::default()
  };


  eframe::run_native(
    "Test GUI App",
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
      name: "Arthur".to_owned(),
      age: 42,
    }

    /// Safely decrements age, respecting the lower bound.
    pub fn decrement_age(&mut self) {
        // Use saturating_sub to prevent underflow panic when age is 0.
        self.age = self.age.saturating_sub(1);
    }
}

impl eframe::App for AppQueryType {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui | {
      ui.heading("Test Application");
      ui.horizontal(|ui| {
        let name_label = ui.label("Name :");
        ui.text_edit_singleline(&mut self.name).labelled_by(name_label.id)
      });

      ui.add(egui::Slider::new(&mut self.age, Self::AGE_RANGE).text("age"));
      if ui.button("Increment").clicked() {
        self.increment_age();
      }

      if ui.button("Decrement").clicked() {
        self.decrement_age();
      }

      ui.label(format!("Name: {}, Age: {}", self.name, self.age));
    });
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_should_default_sensibly() {
        let app = AppQueryType::default();
        assert_eq!(app.name, "");
        assert_eq!(app.age, 0);
    }

    #[test]
    fn increment_age_should_increase_age_by_one() {
        let mut app = AppQueryType { name: "".to_string(), age: 10 };
        app.increment_age();
        assert_eq!(app.age, 11);
    }

    #[test]
    fn decrement_age_should_decrease_age_by_one() {
        let mut app = AppQueryType { name: "".to_string(), age: 10 };
        app.decrement_age();
        assert_eq!(app.age, 9);
    }

    #[test]
    fn decrement_age_should_not_underflow() {
        let mut app = AppQueryType { name: "".to_string(), age: 0 };
        app.decrement_age();
        assert_eq!(app.age, 0, "Age should not go below 0");
    }

    #[test]
    fn increment_age_should_not_exceed_slider_max() {
        let mut app = AppQueryType { name: "".to_string(), age: 100 };
        app.increment_age();
        assert_eq!(app.age, 100, "Age should not exceed 100");
    }
}
