pub extern crate piston;
pub extern crate graphics;
pub extern crate glutin_window;
pub extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };

use ::view::Views;
use ::view::map_view::MapView;
use ::model::Model;
use ::model::input::Input;

pub struct Controller {
    views: Views,
    model: Model
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

impl Controller {
    pub fn initialize_controller() -> Controller {
        
        Controller {
            views: ::view::Views::new(),
            model: ::model::Model::new()
        }
    }

    pub fn run(&mut self) -> Result<bool, String> {
        let opengl = OpenGL::V3_2;
        
        let mut window : Window = WindowSettings::new("Chi", [1024, 768]).opengl(opengl).exit_on_esc(true).build().expect("Unable to initialize OpenGL context");
        let mut events = Events::new(EventSettings::new().lazy(true));
        let mut gfx = GlGraphics::new(opengl);

        

        self.views.add_view(Box::new(MapView::new()));

        while let Some(e) = events.next(&mut window) {
            if !self.handle_input(&e) {
                if let Some(args) = e.render_args() {
                    self.views.notify(&self.model, &mut window, &mut gfx, args);
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