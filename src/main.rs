use gtk::prelude::*;
use relm4::prelude::*;

struct App {
    counter: u8,
}

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            init_for_window,
            set_layer: gtk4_layer_shell::Layer::Overlay,
            auto_exclusive_zone_enable,
            set_margin: (gtk4_layer_shell::Edge::Left, 40),
            set_anchor: (gtk4_layer_shell::Edge::Right, true),
            set_anchor: (gtk4_layer_shell::Edge::Top, false),
            set_anchor: (gtk4_layer_shell::Edge::Bottom, true),
            set_title: Some("Simple app"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => Msg::Increment,
                },

                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked => Msg::Decrement,
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                }
            }
        }
    }

    // Initialize the component.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App { counter };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            Msg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<App>(0);
}

trait LayerShell {
    fn init_layer_shell(&self);
    fn set_layer(&self, layer: gtk4_layer_shell::Layer);
    fn auto_exclusive_zone_enable(&self);
    fn set_margin(&self, edge: gtk4_layer_shell::Edge, margin_size: i32);
    fn set_anchor(&self, edge: gtk4_layer_shell::Edge, anchor_to_edge: bool);
}

impl LayerShell for relm4::gtk4::Window {
    fn init_layer_shell(&self) {
        gtk4_layer_shell::init_for_window(&self);
    }
    fn set_layer(&self, layer: gtk4_layer_shell::Layer) {
        gtk4_layer_shell::set_layer(&self, layer);
    }
    fn auto_exclusive_zone_enable(&self) {
        gtk4_layer_shell::auto_exclusive_zone_enable(&self);
    }
    fn set_margin(&self, edge: gtk4_layer_shell::Edge, margin_size: i32) {
        gtk4_layer_shell::set_margin(&self, edge, margin_size);
    }
    fn set_anchor(&self, edge: gtk4_layer_shell::Edge, anchor_to_edge: bool) {
        gtk4_layer_shell::set_anchor(&self, edge, anchor_to_edge);
    }
}
