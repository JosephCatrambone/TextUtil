
use eframe::{self, egui};
use eframe::egui::ScrollArea;

const TITLE: &'static str = "TextUtil";

// Exported for use in the main.rs,
// but wrapped in a lib so that we can do benches and such.
pub fn run_app() {
	let options = eframe::NativeOptions {
		drag_and_drop_support: true,
		initial_window_size: Some(egui::vec2(320.0, 240.0)),
		..Default::default()
	};
	eframe::run_native(
		TITLE,
		options,
		Box::new(|cc| Box::new(MainApp::new(cc))),
	)
}


#[derive(Debug)]
struct MainApp {
	text: String,
}

impl MainApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);
		Self {
			text: "Edit this text field if you want".to_owned(),
		}
	}
}

impl eframe::App for MainApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			let layout = egui::Layout::top_down(egui::Align::LEFT).with_main_justify(true);
			ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
				//ui.add()
			})
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			let layout = egui::Layout::centered_and_justified(egui::Direction::TopDown);
			//ui.heading(TITLE);
			ui.allocate_ui_with_layout(ui.available_size(), layout, |ui|{
				ScrollArea::both()
					//.auto_shrink([false; 2])
					.show(ui, |ui|{
						ui.text_edit_multiline(&mut self.text);
					});
			});
		});

		/*
		if ctx.input(|i| i.key_pressed(egui::Key::A)) {}
		if ctx.input(|i| i.key_down(egui::Key::A)) {
			ui.ctx().request_repaint(); // make sure we note the holding.
		}
		if ctx.input(|i| i.key_released(egui::Key::A)) {}
		*/
	}
}

fn setup_custom_fonts(ctx: &egui::Context) {
	// Start with the default fonts (we will be adding to them rather than replacing them).
	let mut fonts = egui::FontDefinitions::default();

	let font_name = "anonymous_pro_minus";

	fonts.font_data.insert(
		font_name.to_owned(),
		egui::FontData::from_static(include_bytes!("../resources/Anonymous Pro Minus.ttf")),
	);

	// Put my font first (highest priority) for proportional text:
	fonts
		.families
		.entry(egui::FontFamily::Proportional)
		.or_default()
		.insert(0, font_name.to_owned());

	// Put my font as last fallback for monospace:
	fonts
		.families
		.entry(egui::FontFamily::Monospace)
		.or_default()
		.push(font_name.to_owned());

	// Tell egui to use these fonts:
	ctx.set_fonts(fonts);
}
