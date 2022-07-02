use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_iced::iced::{
    widget::{button, Button, Text},
    Element, Program,
};
use bevy_iced::{IcedAppExtensions, IcedPlugin};
use iced_aw::{color_picker, ColorPicker};

#[derive(Debug, Clone)]
pub enum UiMessage {
    OpenColorPicker,
    CloseColorPicker,
    ColorPicked(iced_native::Color),
}

#[derive(Default)]
pub struct MainUi {
    color_picker_state: color_picker::State,
    color_picker_btn_state: button::State,
}

impl Program for MainUi {
    type Renderer = bevy_iced::iced_wgpu::Renderer;
    type Message = UiMessage;

    fn update(&mut self, msg: UiMessage) -> iced_native::Command<UiMessage> {
        match msg {
            UiMessage::OpenColorPicker => self.color_picker_state.show(true),
            UiMessage::CloseColorPicker => self.color_picker_state.show(false),
            UiMessage::ColorPicked(_) => self.color_picker_state.show(false),
        }
        iced_native::Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message, Self::Renderer> {
        ColorPicker::new(
            &mut self.color_picker_state,
            Button::new(&mut self.color_picker_btn_state, Text::new("Pick color"))
                .on_press(UiMessage::OpenColorPicker),
            UiMessage::CloseColorPicker,
            UiMessage::ColorPicked,
        )
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
