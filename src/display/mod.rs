pub mod cli_display;

pub trait Display {

    fn new() -> Self;
    fn input(&mut self);
    fn render(&self);
    fn get_state(&self) -> &dyn State<Self>;
    fn set_state(&mut self, state: Option<Box<dyn State<Self>>>);
    fn get_exit_flag(&self) -> bool;
}

pub trait State<T> where T: Display {

    fn render(&self);
    fn input(&mut self, display: &mut T) -> bool; 
}

