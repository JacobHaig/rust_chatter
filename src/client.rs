use eframe::egui;
use egui::{Align, TextEdit};
use std::net::TcpStream;
use std::sync::Arc;

use crate::message::{Message, User};
use crate::request::Request;
use crate::response::Response;
use crate::{network, Args};

/// client() is the main function for the client.
pub fn client(args: Arc<Args>) {
    let address = format!("{}:{}", args.address, args.port);

    // Connect to the server and get a stream
    let connection = TcpStream::connect(address).expect("Could not connect to server.");

    let user = User {
        username: args.username.to_owned(),
        id: 0,
    };

    let app = App {
        user_list: vec![],

        connection: Arc::new(connection),
        message_list: vec![],
        message_box_value: "".to_owned(),

        update_interval: 1.0f32,
        username: args.username.to_owned(),
    };

    let _: Response = network::send_get(Request::AddUser(user), app.connection.clone());

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Chatter", native_options, Box::new(|_cc| Box::new(app))).unwrap();
}

pub struct App {
    connection: Arc<TcpStream>,

    message_box_value: String,
    username: String,

    user_list: Vec<User>,
    message_list: Vec<Message>,

    update_interval: f32,
}

impl eframe::App for App {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let user = User {
            username: self.username.to_owned(),
            id: 0,
        };

        let _: Response = network::send_get(Request::RemoveUser(user), self.connection.clone());
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
            let response: Response = network::send_get(request, Arc::clone(&self.connection));

            if let Response::Messages(messages) = response {
                self.message_list = messages;
            }

            // Update Users
            let request = Request::GetUsers();
            let response: Response = network::send_get(request, Arc::clone(&self.connection));

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
                    TextEdit::singleline(&mut self.message_box_value)
                        .hint_text("Enter your message"), // .clip_text(true),
                );

                // When the user presses enter, we send the message to the server.
                if ui.button("send it").clicked() {
                    let message =
                        Message::new(self.username.clone(), self.message_box_value.clone());
                    let request = Request::AddMessage(message);

                    self.message_box_value = String::new();

                    network::send(request, Arc::clone(&self.connection));
                    let _response: Option<Response> = network::get(Arc::clone(&self.connection));
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
