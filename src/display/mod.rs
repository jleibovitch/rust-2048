pub mod cli_display;

pub trait Display {

    fn new() -> Self;
    fn input(&self, state: &mut super::grid::Grid) -> bool;
    fn render(&self, state: &super::grid::Grid);

}
