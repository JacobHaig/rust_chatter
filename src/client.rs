use eframe::{egui, epi};
use std::sync::Arc;

use crate::message::Message;
use crate::request::Request;
use crate::response::Response;
use crate::{networking, Args};

/// client is a function that handles the connection and
/// reads the messages from the stream then print the message
/// to the screen.
pub fn client(args: Arc<Args>) {
    let address = format!("{}:{}", args.address, args.port);

    // Connect to the server and get a stream
    let conn = match std::net::TcpStream::connect(address) {
        Ok(conn) => conn,
        Err(_) => {
            println!("Could not connect to server. ");
            // Quit the program -- This is not a graceful way to quit.
            std::process::exit(0);
        }
    };

    let app = App {
        _user_list: vec![],
        // label: "Hello World!".to_owned(),
        // value: 2.7,
        conn: Arc::new(std::sync::Mutex::new(conn)),
        message_list: vec![],
        text_value: "Enter a message".to_owned(),

        update_interval: 1.0f32,
        username: args.username.to_owned(),
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

// write_and_read is a function that writes a message to the stream
// and then reads the response from the stream. This is a quick and
// dirty way to send a message to the server and get a response fast.
fn write_and_read_connection(
    conn: Arc<std::sync::Mutex<std::net::TcpStream>>,
    request: Request,
) -> Option<Response> {
    let bytes = bincode::serialize(&request).unwrap();
    networking::write_to_connection(&bytes, Arc::clone(&conn));

    let bytes_result = networking::read_from_connection(Arc::clone(&conn));
    if bytes_result.is_some() {
        let bytes = bytes_result.unwrap();
        let response: Response = bincode::deserialize(&bytes).unwrap();

        return Some(response);
    }

    None
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    // label: String,
    _user_list: Vec<String>,

    // this how you opt-out of serialization of a member
    // #[cfg_attr(feature = "persistence", serde(skip))]
    // value: f32,
    conn: Arc<std::sync::Mutex<std::net::TcpStream>>,
    message_list: Vec<Message>,
    text_value: String,

    update_interval: f32,
    username: String,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // After the interval, we will send a request to the server to get the latest messages.
        self.update_interval -= egui::InputState::default().unstable_dt;
        if self.update_interval <= 0.0 {
            self.update_interval = 10.0;

            let request = Request::LastMessages(10);
            let response = write_and_read_connection(Arc::clone(&self.conn), request);

            if let Some(Response::Message(m)) = response {
                for message in &m {
                    println!("{}", message);
                }

                self.message_list = m;
            }
        }

        // This panel is the top panel and shows the menu bar.
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        // This panel is meant to show the currently connected users.
        egui::SidePanel::left("user_panel").show(ctx, |ui| {
            ui.heading("Users");

            ui.label("This is work");
            ui.label("in progress");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            // if ui.button("Decrment").clicked() {
            //     *value -= 1.0;
            // }

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            //     ui.add(
            //         egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
            //     );
            // });
        });

        // This panel is meant to show the list of messages.
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Messages");
            for message in &self.message_list {
                ui.label(message.to_string());
            }

            // ui.hyperlink("https://github.com/emilk/egui_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/egui_template/blob/master/",
            //     "Source code."
            // ));
            egui::warn_if_debug_build(ui);
        });

        // This panel is meant to sent messages to the server.
        egui::TopBottomPanel::bottom("input_area").show(ctx, |ui| {
            ui.heading("Enter your message");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.text_value);

                // When the user presses enter, we send the message to the server.
                if ui.button("send it").clicked() {
                    // println!("Sending '{}'", self.text_value);

                    let message = Message::new(self.text_value.clone(), self.username.clone());
                    let request = Request::AddMessage(message);

                    let _response = write_and_read_connection(Arc::clone(&self.conn), request);
                    // println!("{:?}", response);
                }
            })
        });

        // This is just an example of how to use the `egui::Window` widget.
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
