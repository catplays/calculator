mod app;
mod calc;

use calc::Calculator;


fn main() {
    console_error_panic_hook::set_once();
    // yew::Renderer::<App>::new().render();
     yew::Renderer::<Calculator>::new().render();
}
