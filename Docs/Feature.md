# Silder
```rust
ui.add(egui::Slider::new(&mut self.age, min_range..=max_range).text("age"));
```

# Button
```rust
if ui.button("Increment").clicked() {
  self.age += 1;
}
```

# Popup
```rust
let openbtn = ui.button("Open");
Popup::menu(&openbtn).close_behavior(PopupCloseBehavior::IgnoreClicks).show(|ui| {
    ui.set_min_width(0.0);
    ui.label("Test Popup Button action button");
    ui.checkbox(&mut self.status.checkbox, "Checkbox");
  }
);
```