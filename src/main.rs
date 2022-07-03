use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_iced::iced::{
    widget::{button, Button, Text},
    Element, Program,
};
use bevy_iced::{IcedAppExtensions, IcedPlugin};

#[derive(Debug, Clone)]
pub enum UiMessage {
    ButtonPressed,
}

#[derive(Default)]
pub struct MainUi {
    button_state: button::State,
}

impl Program for MainUi {
    type Renderer = bevy_iced::iced_wgpu::Renderer;
    type Message = UiMessage;

    fn update(&mut self, msg: UiMessage) -> iced_native::Command<UiMessage> {
        match msg {
            UiMessage::ButtonPressed => println!("test"),
        }
        iced_native::Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        Button::new(&mut self.button_state, Text::new("Test Button"))
            .on_press(UiMessage::ButtonPressed)
            .into()
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(IcedPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_non_send_program(MainUi::default())
        .run();
}
