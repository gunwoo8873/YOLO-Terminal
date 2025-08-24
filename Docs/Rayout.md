# Horizontal

```rust
ui.horizontal(|ui| {
  let name_label = ui.label("Name :");
  ui.text_edit_singleline(&mut self.name).labelled_by(name_label.id)
});
```