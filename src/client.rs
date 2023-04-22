use eframe::egui;
use egui::{Align, TextEdit};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::message::{Message, User};
use crate::request::Request;
use crate::response::Response;
use crate::{network, Args};

/// client is a function that handles the connection and
/// reads the messages from the stream then print the message
/// to the screen.
pub fn client(args: Arc<Args>) {
    let address = format!("{}:{}", args.address, args.port);

    // Connect to the server and get a stream
    let conn = TcpStream::connect(address).expect("Could not connect to server.");

    let user = User {
        username: args.username.to_owned(),
        id: 0,
    };

    let app = App {
        user_list: vec![],

        conn: Arc::new(Mutex::new(conn)),
        message_list: vec![],
        text_value: "".to_owned(),

        update_interval: 1.0f32,
        username: args.username.to_owned(),
    };

    let _: Response = network::send_get(Request::AddUser(user), app.conn.clone());

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Chatter", native_options, Box::new(|_cc| Box::new(app))).unwrap();
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    user_list: Vec<User>,

    // this how you opt-out of serialization of a member
    // #[cfg_attr(feature = "persistence", serde(skip))]
    // value: f32,
    conn: Arc<Mutex<TcpStream>>,
    message_list: Vec<Message>,
    text_value: String,

    update_interval: f32,
    username: String,
}

impl eframe::App for App {
    /// Called by the framework to load old app state (if any).
    // #[cfg(feature = "persistence")]
    // fn setup(
    //     &mut self,
    //     _ctx: &egui::CtxRef,
    //     _frame: &mut epi::Frame<'_>,
    //     storage: Option<&dyn epi::Storage>,
    // ) {
    //     if let Some(storage) = storage {
    //         *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    //     }
    // }

    // /// Called by the frame work to save state before shutdown.
    // #[cfg(feature = "persistence")]
    // fn save(&mut self, storage: &mut dyn epi::Storage) {
    //     epi::set_value(storage, epi::APP_KEY, self);
    // }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let user = User {
            username: self.username.to_owned(),
            id: 0,
        };

        let _: Response = network::send_get(Request::RemoveUser(user), self.conn.clone());
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // self.update_data();

        // After the interval, we will send a request to the server to get the latest messages.
        self.update_interval -= egui::InputState::default().unstable_dt;
        if self.update_interval <= 0.0 {
            self.update_interval = 1.0;

            // I really want to make these multithreaded/async but dont want to
            // change the connection to an arc<Mutex<TcpStream>>. I hope to
            // figure this out later.

            // Update Messages
            let request = Request::GetMessages();
            let response: Response = network::send_get(request, Arc::clone(&self.conn));

            if let Response::Messages(m) = response {
                // for message in &m {
                //     println!("{}", message);
                // }
                self.message_list = m;
            }

            // Update Users
            let request = Request::GetUsers();
            let response: Response = network::send_get(request, Arc::clone(&self.conn));

            if let Response::Users(users) = response {
                self.user_list = users;
            }
        }

        // This panel is the top panel and shows the menu bar.
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                ui.separator();

                egui::menu::menu_button(ui, "Help", |ui| {
                    if ui.button("About").clicked() {
                        // This should open a new window with the about information.
                        frame.close();
                    }
                });

                ui.separator();

                // Float this menu to the right
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        // This panel is meant to show the currently connected users.
        egui::SidePanel::left("user_panel").show(ctx, |ui| {
            ui.heading("Users");

            ui.separator();

            for user in &self.user_list {
                ui.label(user.to_string());
            }

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

            ui.separator();

            // Scrollable area
            // let i = ui.available_width();
            // println!("{i}");

            // ui.set_width(ui.available_width());

            // ui.set_height(ui.available_height() - 100.0);

            // ui.hyperlink("https://github.com/emilk/egui_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/egui_template/blob/master/",
            //     "Source code."
            // ));

            ui.vertical(|ui| {
                ui.set_max_height(ui.available_height() - 25.0);

                egui::scroll_area::ScrollArea::new([false, true])
                    // .max_width(f32::INFINITY)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());

                        for message in &self.message_list {
                            ui.label(message.to_string());
                        }
                    });
            });

            ui.separator();

            // This panel is meant to sent messages to the server.
            // egui::TopBottomPanel::bottom("input_area").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.text_value).hint_text("Enter your message"), // .clip_text(true),
                );

                // When the user presses enter, we send the message to the server.
                if ui.button("send it").clicked() {
                    let message = Message::new(self.username.clone(), self.text_value.clone());
                    let request = Request::AddMessage(message);

                    self.text_value = String::new();

                    network::send(request, Arc::clone(&self.conn));
                    let _response: Option<Response> = network::get(Arc::clone(&self.conn));
                }
            });
            // });
        });
        // // This is just an example of how to use the `egui::Window` widget.
        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally chose either panels OR windows.");
        //     });
        // }
    }
}

// impl App {
//     fn update_data(&mut self) {
//         // After the interval, we will send a request to the server to get the latest messages.
//         self.update_interval -= egui::InputState::default().unstable_dt;

//         if self.update_interval <= 0.0 {
//             self.update_interval = 1.0;

//             // Update Messages
//             let request = Request::LastMessages(10);
//             let response: Response = network::send_get(request, Arc::clone(&self.conn));

//             if let Response::Messages(m) = response {
//                 for message in &m {
//                     println!("{}", message);
//                 }

//                 self.message_list = m;
//             }

//             // Update Users
//             let request = Request::GetUsers();
//             let response: Response = network::send_get(request, Arc::clone(&self.conn));

//             if let Response::Users(u) = response {
//                 for user in &u {
//                     println!("{}", user);
//                 }

//                 self._user_list = u;
//             }
//         }
//     }
// }
