#[macro_use]
extern crate vdom;
extern crate vdom_renderers_stdweb;

use std::default::Default;
use vdom::*;
use vdom_renderers_stdweb::*;

#[derive(Clone, Debug)]
pub struct Model {
    count: u32,
}

impl Default for Model {
    fn default() -> Self {
        Model { count: 0 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Actions {
    DoSomethingClicked(u32),
}

impl Component<Actions> for Model {
    fn render(&self) -> NodeTypes<Actions> {
        div(
            &[],
            &[
                span(&[], &[text(format!("{:}", self.count))]),
                button(
                    &[on_click!(
                        // TODO: add event tag for ident here in macro to enable event data extraction within this macro invokation?
                        Actions::DoSomethingClicked(self.count)
                    )],
                    &[text("Do Something")],
                ),
            ],
        )
    }

    fn handle(&mut self, action: Actions) {
        match action {
            Actions::DoSomethingClicked(last) => self.count = last + 1,
        }
    }
}

fn main() {
    println!("Stuff");
    match App::<Model, Actions>::mount_to_body() {
        Ok(app) => app.exec(),
        Err(err) => println!("Err [{:?}]", err),
    }
}
