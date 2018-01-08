use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, TextureSettings, Filter };
use std::path::Path;

use ::view::Views;
use ::view::map_view::MapView;
use ::view::console_view::ConsoleView;
use ::model::Model;
use ::model::input::Input;

pub struct Controller<'a> {
    views: Views<'a>,
    model: Model,
}
///this function does the dirty work of turning keyboard input
///into input the model knows to ingest and properly update state with
///for any controller made, this is necessary 
pub fn key_to_input(key: Key) -> Input {
    match key {
        Key::Up | Key::NumPad8 => Input::Up,
        Key::Down | Key::NumPad2 => Input::Down,
        Key::Left | Key::NumPad4 => Input::Left,
        Key::Right | Key::NumPad6 => Input::Right,
        Key::Escape => Input::Exit,
        _ => Input::UnrecognizedInput
    }
}

impl<'a> Controller<'a> {
    pub fn run_client_application() -> Result<bool, String> {
        let opengl = OpenGL::V3_2;
        
        let mut window : Window = WindowSettings::new("Chi", [1024, 768]).opengl(opengl).exit_on_esc(true).build().expect("Unable to initialize OpenGL context");
        //as we add 'idle' animations we want to remove the lazy true, but for now limiting redraw requests sounds dope:
        let mut events = Events::new(EventSettings::new().lazy(true));
        //(move to ggez if we don't care about lazy? worth consideration..)
        let mut gfx = GlGraphics::new(opengl);
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);

        let mut controller = Controller {
            model: ::model::Model::new(),
            views: Views::new(Path::new("./assets/kato.ttf"), texture_settings),
        };
        controller.views.addView(Box::new(MapView::new()));
        controller.views.addView(Box::new(ConsoleView::new()));
        controller.run(events, window, gfx)

    }

    pub fn run(&mut self, mut events: Events, mut window: Window, mut gfx: GlGraphics) -> Result<bool, String> {

        while let Some(e) = events.next(&mut window) {
            if !self.handle_input(&e) {
                //TODO the following shouldn't run if no changes are really identified. 
                if let Some(args) = e.render_args() {
                    match self.views.notify(&self.model, &mut window, &mut gfx, args) {
                        Err(view_error) => error!("View notification failed."),
                        _ => ()//no-op if it worked
                    }
                }
            }
        }
        Ok(true)
    }

    fn handle_input(&mut self, event: &Event) -> bool {
        if let Some(button_args) = event.press_args() {
            //handle button press
            self.button_pressed(button_args);
            return true;
        } else if let Some(resize_args) = event.resize_args() {
            self.window_resized(resize_args[0], resize_args[1]);
            return true;
        }
        false
        
    }
    fn window_resized(&mut self, width: u32, height: u32) {
        debug!("resized a window!");
    }
    fn button_pressed(&mut self, button_enum: Button) {
        match button_enum {
            Button::Keyboard(button) => self.pushed_key(button),
            _ => ()
        }
    }

    fn pushed_key(&mut self, keyboard_button: Key) {
        self.model.player_input(key_to_input(keyboard_button));

    }


}