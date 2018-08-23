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
pub enum Events {
    DoSomethingClicked(u32),
}

impl Component<Events> for Model {
    fn render(&self) -> NodeTypes<Events> {
        div(
            &[  class("special")
            ],
            &[  span(&[], &[text(format!("{:}", self.count))]),
                button(
                    &[ on_click!(
                            // TODO: add event tag for ident here in macro to enable event data extraction within this macro invokation?
                            Events::DoSomethingClicked(self.count)
                        )
                    ],
                    &[ text("Do Something"),
                    ],
                ),
            ],
        )
    }

    fn handle(&mut self, action: Events) {
        match action {
            Events::DoSomethingClicked(last) => self.count = last + 1,
        }
    }
}

fn main() {
    println!("Stuff");
    match App::<Model, Events>::mount_to_body() {
        Ok(app) => app.exec(),
        Err(err) => println!("Err [{:?}]", err),
    }
}
