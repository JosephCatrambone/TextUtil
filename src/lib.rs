use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use boa_engine::Context as BEContext;
use boa_engine::builtins::JsArgs;
use boa_engine::JsResult;
use boa_engine::JsValue;
use boa_engine::property::Attribute as BEAttribute;
use boa_engine::object::ObjectInitializer;
use eframe::{self, egui};
use eframe::egui::ScrollArea;
use walkdir::WalkDir;

const TITLE: &'static str = "TextUtil";

// Exported for use in the main.rs,
// but wrapped in a lib so that we can do benches and such.
pub fn run_app() {
	let options = eframe::NativeOptions {
		drag_and_drop_support: true,
		initial_window_size: Some(egui::vec2(640.0, 480.0)),
		multisampling: 1,
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
	force_command_focus: bool,
	command: String,
	text: String,
	plugins: HashMap<String, String>, // BEContext.parse() -> statement list -> BEContext.compile() -> JsResult<Gc<CodeBlock>> -> BEContext.execute().
}

impl MainApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		setup_custom_fonts(&cc.egui_ctx);
		let mut app = Self {
			force_command_focus: true,
			command: "".to_owned(),
			text: "".to_owned(),
			plugins: HashMap::new(),
		};
		app.enumerate_plugins();
		app
	}

	fn dumb_run_self_contained(&mut self, plugin_code: &str) {
		let mut context = BEContext::default();

		let startup_plugin = context.eval(plugin_code);
		if startup_plugin.is_err() {
			println!("{:?}", startup_plugin.as_ref().err());
			println!("{:?}", &startup_plugin.as_ref().err().unwrap());
			println!("{:?}", &startup_plugin.as_ref().err().unwrap().to_string(&mut context).unwrap());
			println!("{:?}", &startup_plugin.as_ref().err().unwrap().to_string(&mut context).unwrap().as_str());
		}

		/*
		let script = r#"
		function test(arg1) {
			if(arg1 != null) {
				return arg1.x;
			}
			return 112233;
		}
		"#;

		// Create an object that can be used in eval calls.
		let arg = ObjectInitializer::new(&mut context)
			.property("x", 12, BEAttribute::READONLY)
			.build();
		context.register_global_property(
			"arg",
			arg,
			BEAttribute::all()
		);

		let value = context.eval("test(arg)").unwrap();

		assert_eq!(value.as_number(), Some(12.0))
		*/

		context.register_global_property::<&str, String>("text_input", self.text.clone(), BEAttribute::READONLY);
		let processed = context.eval("main(text_input)");
		match processed {
			Ok(val) => {
				self.text = val.as_string().unwrap().to_string()
			},
			Err(js_error) => {
				println!("{}", js_error.to_string(&mut context).unwrap().as_str());
			}
		}
	}

	fn invoke_plugin(&mut self, js_code: &str) {
		let mut js_context = BEContext::default();
		js_context.register_global_builtin_function("say_hello", 1, say_hello);
		js_context.register_global_property("MY_PROJECT_VERSION", "1.0.0", BEAttribute::all());

		let mut context = BEContext::default();
		match context.eval(js_code) {
			Ok(res) => {
				let out_str = res.to_string(&mut context).unwrap();
			}
			Err(e) => {
				// Pretty print the error
				eprintln!("Uncaught {}", e.display());
			}
		}
	}

	fn enumerate_plugins(&mut self) {
		//for entry in WalkDir::new("plugins").into_iter().filter_map(|e| e.ok()).filter(|e| e.path().ends_with("js")) {
		for entry in WalkDir::new("plugins").into_iter().filter_map(|e| e.ok()) {
			if let Ok(file) = File::open(entry.path()) {
				let mut buf_reader = BufReader::new(file);
				let mut contents = String::new();
				let read_op = buf_reader.read_to_string(&mut contents);
				if read_op.is_ok() {
					let plugin_name = entry.path().file_stem().unwrap().to_str().expect("Couldn't convert from OS string to string.  This seems impossible.").to_lowercase();
					println!("Loaded plugin with name {}", &plugin_name);
					self.plugins.insert(plugin_name, contents);
				}
				//println!("{}", entry.path().display());
			}
		}
	}
}

impl eframe::App for MainApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			//let layout = egui::Layout::top_down(egui::Align::LEFT).with_main_justify(true);
			let layout = egui::Layout::left_to_right(egui::Align::LEFT).with_main_justify(true);
			ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
				//ui.text_edit_singleline(&mut self.command);
				let response = ui.add(egui::TextEdit::singleline(&mut self.command));

				// If Shift+Space is hit, focus on this command bar.
				if self.force_command_focus {
					response.request_focus();
					self.force_command_focus = false;
				}

				if response.changed() {
					// TODO: Autocomplete.
				}

				//if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
				//if response.lost_focus() && ctx.input().key_pressed(egui::Key::Enter) {
				if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
					let cmd = self.command.clone();
					if let Some(code) = self.plugins.get(&cmd) {
						let code = code.clone();
						//self.invoke_plugin(&cmd);
						self.dumb_run_self_contained(&code);
					} else {
						println!("No plugin with name {} found.", &cmd);
					}
				}
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

		if ctx.input().modifiers.shift && ctx.input().key_released(egui::Key::Space) {
			self.force_command_focus = true;
		}

		/*
		if ctx.input(|i| i.key_pressed(egui::Key::A)) {}
		if ctx.input().key_released(egui::Key::Enter) {
		//ui.ctx().request_repaint(); // make sure we note the holding.
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

fn say_hello(_this: &JsValue, args: &[JsValue], context: &mut BEContext) -> JsResult<JsValue> {
	// say_hello("Rust");
	let name = args.get_or_undefined(0);

	if name.is_undefined() {
		println!("Hello World!");
	} else {
		println!("Hello {}!", name.to_string(context)?);
	}

	Ok(JsValue::undefined())
}

