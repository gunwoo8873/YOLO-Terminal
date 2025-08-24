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