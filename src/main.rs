use raylib::prelude::*;
use matematika_rs::sistem::aljabar::SistemPersamaan;

fn draw_rounded_rect_with_border(
    d: &mut RaylibDrawHandle,
    rect: Rectangle,
    roundness: f32,
    segments: i32,
    border_thickness: f32,
    border_color: Color,
    fill_color: Color,
) {
    // Draw outer (border)
    d.draw_rectangle_rounded(&rect, roundness, segments, border_color);

    // Create inner rectangle (shrunk to fit inside border)
    let inner_rect = Rectangle::new(
        rect.x + border_thickness,
        rect.y + border_thickness,
        rect.width - 2.0 * border_thickness,
        rect.height - 2.0 * border_thickness,
    );

    // Draw inner (fill)
    d.draw_rectangle_rounded(&inner_rect, roundness, segments, fill_color);
}

fn main() {
    let (mut rl_handle, rl_thread) = raylib::init()
        .size(860, 700)
        .title("spldv-paijo")
        .build();

    rl_handle.set_exit_key(None);
    rl_handle.set_target_fps(60);

    let font_header = rl_handle
        .load_font_ex(&rl_thread, "src/resources/SF-Pro-Display-Heavy.otf", 55, None)
        .expect("Failed to load header font");

    let font_normal = rl_handle
        .load_font_ex(&rl_thread, "src/resources/SF-Pro-Display-Bold.otf", 30, None)
        .expect("Failed to load normal font");

    let bg = Color::new(251, 241, 199, 255);
    let text_color = Color::new(40, 40, 40, 255);
    let header_color = Color::new(181, 118, 20, 255);

    let mut input_fields = [
        String::new(), // a1
        String::new(), // b1
        String::new(), // c1
        String::new(), // a2
        String::new(), // b2
        String::new(), // c2
    ];

    let mut selected_index = 0;
    let mut result: Option<(f64, f64)> = None;

    while !rl_handle.window_should_close() {
        let mouse = rl_handle.get_mouse_position();

        if rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let field_width = 80.0;
            let field_height = 46.0;
            let spacing = 5.0;
            let start_x = (860.0 - (field_width * 3.0 + spacing * 2.0 + 80.0)) / 2.0;
            let mut y = 260.0;

            for row in 0..2 {
                for col in 0..3 {
                    let i = row * 3 + col;

                    // Correct spacing logic
                    let x = match col {
                        0 => start_x,
                        1 => start_x + field_width + spacing + 40.0, // skip space for 'x'
                        2 => start_x + (field_width + spacing) * 2.0 + 80.0, // skip space for 'x' and 'y' + '='
                        _ => continue,
                    };

                    let rect = Rectangle::new(x, y, field_width, field_height);
                    if rect.check_collision_point_rec(mouse) {
                        selected_index = i;
                    }
                }
                y += 100.0;
            }
        }

        if rl_handle.is_key_pressed(KeyboardKey::KEY_TAB) {
            if rl_handle.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
                || rl_handle.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT)
            {
                selected_index = if selected_index == 0 { 5 } else { selected_index - 1 };
            } else {
                selected_index = (selected_index + 1) % 6;
            }
        }

        if rl_handle.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            input_fields[selected_index].pop();
        }

        while let Some(c) = rl_handle.get_char_pressed() {
            if c >= ' ' && c <= '~' {
                input_fields[selected_index].push(c);
            }
        }

        if rl_handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
            let parsed: Vec<f64> = input_fields
                .iter()
                .map(|s| s.parse::<f64>().unwrap_or(0.0))
                .collect();

            result = SistemPersamaan::spldv(
                parsed[0], parsed[1], parsed[2],
                parsed[3], parsed[4], parsed[5],
            );
        }

        // Input field size
        let field_width = 80;
        let field_height = 46;
        let spacing = 10;
        let start_x = (860 - (field_width * 4 + spacing * 3 + 110)) / 2;
        let mut y = 260;

        let hitung_rect = Rectangle::new(start_x as f32, (y + 175) as f32, 180.0, 46.0);
        if rl_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && hitung_rect.check_collision_point_rec(mouse)
        {
            let parsed: Vec<f64> = input_fields
                .iter()
                .map(|s| s.parse::<f64>().unwrap_or(0.0))
                .collect();

            result = SistemPersamaan::spldv(
                parsed[0], parsed[1], parsed[2],
                parsed[3], parsed[4], parsed[5],
            );
        }

        let mut d = rl_handle.begin_drawing(&rl_thread);
        d.clear_background(bg);

        d.draw_text_ex(&font_header, "KALKULATOR SPLDV LORDPAIJO", Vector2::new((start_x / 2 - 55) as f32, 140.0), 55.0, 2.0, header_color);

        // Persamaan 1
        d.draw_text_ex(&font_normal, "Persamaan 1:", Vector2::new(start_x as f32, (y - 40) as f32), 30.0, 2.0, text_color);

        for i in 0..3 {
            let is_selected = selected_index == i;
            let field_color = if is_selected { Color::LIGHTGRAY } else { Color::DARKGRAY };
            let rect_x = start_x + i * (field_width + spacing + 80);
            d.draw_rectangle(rect_x as i32, y, field_width.try_into().unwrap(), field_height, field_color);
            d.draw_text_ex(&font_normal, &input_fields[i], Vector2::new((rect_x + 4) as f32, (y + 8) as f32), 30.0, 2.0, text_color);

            // draw symbols
            if i == 0 {
                d.draw_text_ex(&font_normal, "x \t+", Vector2::new((rect_x + field_width + 5) as f32, (y + 8) as f32), 30.0, 2.0, text_color);
            } else if i == 1 {
                d.draw_text_ex(&font_normal, "y \t=", Vector2::new((rect_x + field_width + 5) as f32, (y + 8) as f32), 30.0, 2.0, text_color);
            }
        }

        y += 100; // move to next row

        // Persamaan 2
        d.draw_text_ex(&font_normal, "Persamaan 2:", Vector2::new(start_x as f32, (y - 40) as f32), 30.0, 2.0, text_color);

        for i in 3..6 {
            let is_selected = selected_index == i;
            let field_color = if is_selected { Color::LIGHTGRAY } else { Color::DARKGRAY };
            let rect_x = start_x + (i - 3) * (field_width + spacing + 80);
            d.draw_rectangle(rect_x as i32, y, field_width.try_into().unwrap(), field_height, field_color);
            d.draw_text_ex(&font_normal, &input_fields[i], Vector2::new((rect_x + 4) as f32, (y + 8) as f32), 30.0, 2.0, text_color);

            if i == 3 {
                d.draw_text_ex(&font_normal, "x \t+", Vector2::new((rect_x + field_width + 5) as f32, (y + 8) as f32), 30.0, 2.0, text_color);
            } else if i == 4 {
                d.draw_text_ex(&font_normal, "y \t=", Vector2::new((rect_x + field_width + 5) as f32, (y + 8) as f32), 30.0, 2.0, text_color);
            }
        }

        // Draw "Hitung" button
        let mouse_pos = d.get_mouse_position();
        let hitung_hovered = hitung_rect.check_collision_point_rec(mouse_pos);
        let hitung_color = if hitung_hovered { Color::new(121, 116, 14, 255) } else { Color::new(152, 151, 26, 255) };
        let hitung_border_color = if hitung_hovered { Color::new(104, 157, 106, 255) } else { Color::new(142, 192, 124, 255) };
        draw_rounded_rect_with_border(&mut d, hitung_rect, 0.75, 10, 3.0, hitung_border_color, hitung_color);
        d.draw_text_ex(&font_normal, "Hitung", Vector2::new(start_x as f32 + 44.0, (y + 83) as f32), 30.0, 2.0, text_color);

        // Result
        let result_text = match result {
            Some((x, y)) => format!("x = {:.2}, y = {:.2}", x, y),
            None => "\n".to_string(),
        };
        d.draw_text_ex(&font_normal, "Hasil:\n", Vector2::new(start_x as f32, 500.0), 30.0, 2.0, text_color);
        d.draw_text_ex(&font_normal, &result_text, Vector2::new(start_x as f32, 530.0), 30.0, 2.0, text_color);
        d.draw_text_ex(&font_normal, "Tab: \t\t\t\t\tcol += 1\nShift+Tab: col -= 1", Vector2::new(600.0, 620.0), 25.0, 2.0, text_color);
    }
}
