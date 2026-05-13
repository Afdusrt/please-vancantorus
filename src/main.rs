use raylib::prelude::*;
use rfd::FileDialog;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::time::Duration;
use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use std::collections::HashMap;
use std::sync::mpsc;

const DEFAULT_CONFIG: &str =
"left=q
right=e
left10=a
right10=d
font_color_hex=FFFFFF
font_background_hex=000000";

fn keys_from_path(path: &PathBuf) -> (char, u32) {
	let name = path.file_stem().unwrap().to_string_lossy();

	let digits: String = name
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
        
	let number = digits.parse::<u32>().unwrap_or(0);
	
	let prefix = name.chars().find(|c| c.is_ascii_alphabetic()).unwrap_or('z');
	
   (prefix, number)
}

fn get_img_paths(folder: PathBuf) -> Vec<PathBuf> {
	let mut paths = Vec::new();
	
	for entry in fs::read_dir(folder).expect("Folder list error") {
		let path = entry.unwrap().path();
		if !path.is_dir() {
			if let Some(extenstion) = path.extension() {
				let extenstion = extenstion.to_string_lossy().to_lowercase();

				if matches!(extenstion.as_str(), "png" | "jpg" | "jpeg" | "bmp" | "gif") {
					paths.push(path);
				}
			}
		}
	}
	
	paths.sort_by_key(|p| keys_from_path(&p));
	
	//paths.sort();
	
	return paths
}

fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread, path: &Path) -> Texture2D {
	let texture = rl.load_texture(thread, path.to_str().unwrap()).expect("Texture load error");
	
	return texture
}

fn draw_file_name(d: &mut RaylibDrawHandle, path: &Path, bounds: Rectangle, size: &mut i32, font: &Font, screen_width: i32, font_colors: (Color, Color), stretch_to_square: &mut bool, windows_always_on_top: &mut bool) {
	let text = path.file_stem().unwrap().to_string_lossy();
	let text_width = font.measure_text(&text, *size as f32, 0.0).x as f32;
	let x_offset = (screen_width as f32 - text_width) / 2.0;
	let (fg, bg) = font_colors;
	d.draw_rectangle_rec(bounds, bg);
	d.draw_text_ex(font, &text, Vector2 {x: x_offset, y: 0.0}, *size as f32, 0.0, fg);
	
	if d.gui_button(Rectangle {x: 0.0, y: 0.0, width: bounds.height / 2.0, height: bounds.height / 2.0}, "+") {
		*size += 1;
	}
	
	if d.gui_button(Rectangle {x: 0.0, y: bounds.height / 2.0, width: bounds.height / 2.0, height: bounds.height / 2.0}, "-") {
		*size -= 1;
	}
	
	//raylib::consts::GuiTextAlignment = raylib::consts::GuiTextAlignment::TEXT_ALIGN_RIGHT;
	d.gui_set_style(raylib::consts::GuiControl::CHECKBOX, GuiControlProperty::TEXT_ALIGNMENT, 2);
	if d.gui_check_box(Rectangle {x: bounds.height / 2.0, y: 0.0, width: bounds.height, height: bounds.height}, "stretch to square", stretch_to_square) {
		//unneeded
	}
	#[cfg(target_os = "windows")]
	{
		d.gui_set_style(raylib::consts::GuiControl::CHECKBOX, GuiControlProperty::TEXT_ALIGNMENT, 0);
		if d.gui_check_box(Rectangle {x: (screen_width as f32) - bounds.height, y: 0.0, width: bounds.height, height: bounds.height}, "always on top", windows_always_on_top) {
			//unneeded
		}
	}
}

fn match_keycode(s: &str) -> Keycode {
    match s.trim().to_lowercase().as_str() {
        "a" => Keycode::A,
        "b" => Keycode::B,
        "c" => Keycode::C,
        "d" => Keycode::D,
        "e" => Keycode::E,
        "f" => Keycode::F,
        "g" => Keycode::G,
        "h" => Keycode::H,
        "i" => Keycode::I,
        "j" => Keycode::J,
        "k" => Keycode::K,
        "l" => Keycode::L,
        "m" => Keycode::M,
        "n" => Keycode::N,
        "o" => Keycode::O,
        "p" => Keycode::P,
        "q" => Keycode::Q,
        "r" => Keycode::R,
        "s" => Keycode::S,
        "t" => Keycode::T,
        "u" => Keycode::U,
        "v" => Keycode::V,
        "w" => Keycode::W,
        "x" => Keycode::X,
        "y" => Keycode::Y,
        "z" => Keycode::Z,

        "left" => Keycode::Left,
        "right" => Keycode::Right,
        "up" => Keycode::Up,
        "down" => Keycode::Down,

        invalid => panic!("Invalid key in config: {}", invalid),
    }
}

#[cfg(target_os = "windows")]
fn set_always_on_top(rl: &mut RaylibHandle, enabled: bool) {
	use windows::Win32::Foundation::HWND;
	use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
	use windows::Win32::UI::WindowsAndMessaging::HWND_TOPMOST;
	use windows::Win32::UI::WindowsAndMessaging::HWND_NOTOPMOST;
	use windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE;
	use windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE;
	
	let hwnd_raw = unsafe { rl.get_window_handle() };
	
	let hwnd = HWND(hwnd_raw);
	
	unsafe {
		match SetWindowPos(
			hwnd,
			if enabled {
				Some(HWND_TOPMOST)
			} else {
				Some(HWND_NOTOPMOST)
			},
			0,
			0,
			0,
			0,
			SWP_NOMOVE | SWP_NOSIZE
		) {
			Ok(_) => {},
			Err(_) => {}
		}
	}
}

fn main() {
	let config_file = match fs::read_to_string("config.txt") {
		Ok(file) => { file },
		Err(e) => { eprint!("Writing default config because: {}", e);
					match fs::write("config.txt", DEFAULT_CONFIG) {
						Ok(_) => {},
						Err(e) => { panic!("failed to write default config: {}", e) }
					}
					match fs::read_to_string("config.txt") {
						Ok(file) => { file },
						Err(_) => { panic!("cant read config.txt even tho it should be there") }
					}
		}
	};
	
	let config: HashMap<String, String> = config_file
		.lines()
			.map(|line| line.split_once('=').expect("Invalid config spec") )
			.map(|(option, value)| (option.to_string(), value.to_string()) )
		.collect();
	
	let file = FileDialog::new()
		.set_title("Select image folder")
		.pick_folder();
		
    let image_folder = match file {
		Some(p) => p, //PathBuf type
		None => { println!("No folder selected."); return; }
	};
	
	let paths = get_img_paths(image_folder);
	if paths.is_empty() {
		eprintln!("No images found.");
		return;
	}
	//println!("{:?}", paths);
	let maximum = paths.len()-1;
	let mut selected_image = 0;
	let mut stretch_to_square = true;
	let mut windows_always_on_top = false;
	let mut last_windows_always_on_top = false;
	
	let (mut rl, thread) = raylib::init()
        .size(640, 640)
        .title("link app")
        .resizable()
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();
	rl.set_target_fps(60);
	
	let mut font_size = 30;
	let font_load_res = config.get("font_load_res").expect("font_load_res not in config").parse::<i32>().expect("font_load_res not i32");
	let font_file = config.get("font").expect("font not in config");
	let font = rl.load_font_ex(&thread, font_file, font_load_res, None).expect("failed to load font");
	
	let mut texture_to_show = load_texture(&mut rl, &thread, &paths[selected_image].clone());
	
	let mut font_bounds = Rectangle {x: 0.0, y: 0.0, width: 0.0, height: 0.0};
	let mut image_bounds = Rectangle {x: 0.0, y: 0.0, width: 0.0, height: 0.0};
	
	//let config_file = fs::read_to_string("config.txt").expect("Config file missing");
	
	
		
	let font_color_hex = Color::from_hex(config.get("font_color_hex").expect("font_color_hex not in config")).expect("failed to parse font_color_hex");
	let font_background_hex = Color::from_hex(config.get("font_background_hex").expect("font_background_hex not in config")).expect("failed to parse font_background_hex");
	let font_colors: (Color, Color) = (font_color_hex, font_background_hex);
	
	let left_keybind = match_keycode(&config.get("left").unwrap());
	let left10_keybind = match_keycode(&config.get("left10").unwrap());
	let right_keybind = match_keycode(&config.get("right").unwrap());
	let right10_keybind = match_keycode(&config.get("right10").unwrap());
	
	let (tx, rx) = mpsc::channel();
	let tx2 = tx.clone();
	
	let device_state = DeviceEventsHandler::new(Duration::from_millis(10))
		.expect("Failed to start event loop");
	let _guard = device_state.on_key_down(move |key| {
		if *key == left_keybind {
			let _ = tx2.send(1);
		}
		if *key == right_keybind {
			let _ = tx2.send(2); //code 2 backward
		}
		if *key == left10_keybind {
			let _ = tx2.send(3); //code 2 backward
		}
		if *key == right10_keybind {
			let _ = tx2.send(4); //code 2 backward
		}
	});
	
    while !rl.window_should_close() {
		while let Ok(msg) = rx.try_recv() {
			match msg {
				1 => { if selected_image > 0 { selected_image -= 1 } else { continue } },
				2 => { if !((selected_image + 1) > maximum) { selected_image += 1 } else { continue } },
				3 => { if selected_image >= 10 { selected_image -= 10 } else { continue } },
				4 => { if !((selected_image + 10) > maximum) { selected_image += 10 } else { continue } },
				_ => { continue }
			}
			//drop(texture_to_show);
			texture_to_show = load_texture(&mut rl, &thread, &paths[selected_image].clone());
			//println!("{:?}", paths[selected_image]);
		}
		
		#[cfg(target_os = "windows")]
        if windows_always_on_top != last_windows_always_on_top {
			set_always_on_top(&mut rl, windows_always_on_top);
			last_windows_always_on_top = windows_always_on_top;
		}
		
		let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        
        let screen_width = d.get_screen_width();
        //let screen_height = d.get_screen_height() as f32;
        //d.set_window_size(screen_width, screen_width + font_size);
        
        font_bounds.x = 0.0;
        font_bounds.y = 0.0;
        font_bounds.width = screen_width as f32;
        font_bounds.height = font_size as f32;
        
        image_bounds.x = 0.0;
        image_bounds.y = font_size as f32;
        image_bounds.width = screen_width as f32;
        image_bounds.height = screen_width as f32;
        
        if !stretch_to_square {
			let aspect_ratio = texture_to_show.height as f32 / texture_to_show.width as f32;

			image_bounds.height = screen_width as f32 * aspect_ratio;
			d.set_window_size(screen_width, (screen_width as f32 * aspect_ratio) as i32 + font_size);
		} else {
			d.set_window_size(screen_width, screen_width + font_size);
		}
        
        draw_file_name(&mut d, &paths[selected_image].clone(), font_bounds, &mut font_size, &font, screen_width, font_colors, &mut stretch_to_square, &mut windows_always_on_top);
        
        //d.draw_texture(&texture_to_show, 0, 0, Color::WHITE);
        d.draw_texture_pro(&texture_to_show, Rectangle {x: 0.0, y: 0.0, width: texture_to_show.width as f32, height: texture_to_show.height as f32}, image_bounds, Vector2 {x: 0.0, y: 0.0}, 0.0, Color::WHITE);
	}
}
