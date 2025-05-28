use iced::Sandbox;
use iced::widget::Text;
use iced::Settings;
use iced::Element;

struct NetworkHunter {
    ipaddress: String,
    port: u16,
    scan_type: String,
}

impl Sandbox for NetworkHunter {
    type Message = ();

    fn new() -> Self {
        NetworkHunter {
            
            ipaddress: String::new(),
            port: 0,
            scan_type: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Network Hunter")
    }

    fn update(&mut self, _message: Self::Message) {
        // Update logic here
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // View logic here
        iced::Text::new("Network Hunter").into()
    }
}



fn main() {
    NetworkHunter::run(Settings::default()).expect("Failed to run the application");
}
