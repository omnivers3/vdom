#[macro_use]
extern crate stdweb;

// #[macro_use]
extern crate vdom;

// use std::rc::Rc;
// use std::cell::RefCell;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};

static STDWEB_INITIALIZED: Once = ONCE_INIT;

use stdweb::web::{
    // NodeType,
    document,
    Element,
    HtmlElement,
    // IElement,
    IEventTarget,
    INode,
    Node,
    IParentNode,
    // window,
};

use vdom::*;

use stdweb::web::event::{
    ConcreteEvent,
    // IEvent,
    // DoubleClickEvent,
    ClickEvent,
    // KeyPressEvent,
    // ChangeEvent,
    // BlurEvent,
    // HashChangeEvent
};

// // Shamelessly stolen from webplatform's TodoMVC example.
// macro_rules! enclose {
//     ( ($( $x:ident ),*) $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
// }

/// `stdweb` doesn't have methods to work with attributes now.
/// this is [workaround](https://github.com/koute/stdweb/issues/16#issuecomment-325195854)
pub fn set_attribute(element: &Element, name: &str, value: &str) {
    js!( @(no_return) @{element}.setAttribute( @{name}, @{value} ); );
}

/// Removes attribute from a element by name.
pub fn remove_attribute(element: &Element, name: &str) {
    js!( @(no_return) @{element}.removeAttribute( @{name} ); );
}

// pub fn map_event<TEvent>(
//     handler: &HandlerTypes,
//     _event: &TEvent,
// ) -> EventDataTypes where
//     TEvent: ConcreteEvent,
// {
//     match handler {
//         HandlerTypes::OnClick => EventDataTypes::Click,
//     }
// }

#[derive(Clone, Debug)]
pub enum Errors {
    MountSelectorError,
    InvalidMountSelector (String),
    AppTerminated,
}

#[derive(Clone)]
pub struct App<TModel, TActions> where
    TModel: Component<TActions>,
    // TActions: 'static + Clone,
    TActions: Clone + std::fmt::Debug,
{
    // model: Rc<Mutex<TModel>>,
    // model: RefCell<TModel>,
    model: Arc<Mutex<TModel>>,
    target: Element,
    vdom: NodeTypes<TActions>,
    // handler: &'static FnMut(TActions) + Send,
}

unsafe impl<TModel, TActions> Send for App<TModel, TActions> where
    TModel: Component<TActions>,
    // TActions: 'static + Clone + std::fmt::Debug,
    TActions: Clone + std::fmt::Debug,
{}

unsafe impl<TModel, TActions> Sync for App<TModel, TActions>where
    TModel: Component<TActions>,
    // TActions: 'static + Clone + std::fmt::Debug,
    TActions: Clone + std::fmt::Debug,
{}


// impl <TModel, TActions> RenderTarget<TActions> for App<TModel, TActions> where
//     TModel: 'static + Clone + Component<TActions>,
//     // TActions: 'static + Clone + std::fmt::Debug,
//     TActions: 'static + Clone + std::fmt::Debug,
// {
//     fn apply_patches(self, patches: PatchTypes<TActions>) {
//         let model = self.model.clone();
//         let target = self.target.clone();
//         // apply_patches(target, patches, Rc::new(| action | {
//         //     println!("handler action: {:?}", action);
//         //     self.model.handle(action);
//         // }));
//         apply_patches(model, target, patches);
//     }
// }

impl <TModel, TActions> App<TModel, TActions> where
    TModel: 'static + Clone + Component<TActions>,
    // TActions: 'static + Clone + std::fmt::Debug,
    TActions: 'static + Clone + std::fmt::Debug,
{
    pub fn mount(target: Element) -> Result<Self, Errors> {
        // let model = Rc::new(Mutex::new(TModel::default()));
        // let model = RefCell::new(TModel::default());
        let model = TModel::default();
        let vdom = model.render();
        let model = Arc::new(Mutex::new(model));
        // let vdom = model.borrow().render();
        Ok (
            App {
                model,
                target,
                vdom,
            }
        )
    }

    pub fn mount_to<T>(selector: T) -> Result<Self, Errors> where
        T: ToString
    {
        let selector = selector.to_string();
        document()
            .query_selector(&selector)
            .map_err(|_| Errors::MountSelectorError)
            .and_then(|e| e.ok_or(Errors::InvalidMountSelector (selector)))
            .and_then(|target| App::mount(target))
    }

    pub fn mount_to_body() -> Result<Self, Errors> {
        App::mount_to("body")
    }

    pub fn exec(self) {
        // Ensure stdweb::initialize has been called
        STDWEB_INITIALIZED.call_once(|| {
            stdweb::initialize();
        });

        let vdom = self.vdom.clone();
        let patches = diff(&NodeTypes::Empty, &vdom);
        println!("Exec Patches: {:?}", patches);
        if let Some (patches) = patches {
            // target.apply_patches(patches);
            let model = self.model.clone();
            let vdom = self.vdom.clone();
            let target = self.target.clone();
            // apply_patches(target, patches, Rc::new(| action | {
            //     println!("handler action: {:?}", action);
            //     self.model.handle(action);
            // }));
            apply_patches(model, vdom, target, patches);
        }
        // apply_vdom(self, NodeTypes::Empty, vdom);

        stdweb::event_loop();
    }
}

// fn apply_patches<TActions, FHandler>(
//     // model: &mut TModel,
//     target: Element,
//     patches: PatchTypes<TActions>,
//     handler: Rc<FHandler>,
// ) where
//     // FHandler: Fn(VActionTarget<TPayload>, HandlerTypes, EventDataTypes),
//     TActions: 'static + Clone + std::fmt::Debug,
//     FHandler: 'static + FnMut(TActions),
// {
fn apply_patches<TModel, TActions>(
    // model: Rc<TModel>,
    // model: &mut TModel,
    // app: &App<TModel, TActions>,
    // model: RefCell<TModel>,
    model: Arc<Mutex<TModel>>,
    vdom: NodeTypes<TActions>,
    target: Element,
    patches: PatchTypes<TActions>,
    // handler: Rc<FHandler>,
) where
    // FHandler: Fn(VActionTarget<TPayload>, HandlerTypes, EventDataTypes),
    TModel: 'static + Clone + Component<TActions>,
    // TActions: 'static + Clone + std::fmt::Debug,
    TActions: 'static + Clone + std::fmt::Debug,
    // FHandler: 'static + FnMut(TActions),
{
    match patches {

        PatchTypes::Empty => {
            println!("Soul crushing emptyness");
        },

        PatchTypes::AddComment ( value ) => {
            println!("\nNOT IMPLEMENTED!!\nAdd Comment: {:?}", value);
        },

        PatchTypes::SetText ( value ) => {
            println!("Set Text: {:?}", value);
            target.set_text_content( &value );
            // match target.node_type() {
            //     NodeType::Element => {
            //         target.set_text_content( &value );
            //     },
            //     NodeType::Text => {
            //         // TODO: Append value to existing?
            //         target.set_text_content( &value );
            //     },
            //     _ => {}
            // }
        }

        PatchTypes::AddElement ( kind, attributes, children ) => {
            println!("Append Node: {:?}", kind);
            let element = document().create_element(&kind).unwrap();
            target.append_child(&element);

            for ( key, value ) in attributes {
                set_attribute(&element, &key, &value);
            }
            for child in children {
                // apply_patches(element.to_owned(), child, handler.clone())
                let model = model.clone();
                let vdom = vdom.clone();
                let element = element.clone();
                apply_patches(model, vdom, element, child);
            }
        }

        PatchTypes::AddHandler ( handler_type, action_target ) => {
            println!("\nApply Handler: [ {:?} ] as [ {:?} ]", handler_type, action_target);
            // let a = action.clone();
            // let handler = handler.clone();
            // let model = model.clone();
            // let app = app.clone();
            // let model = model.borrow_mut();
            let model = model.clone();
            let arc_model = Arc::clone(&model);
            let vdom = vdom.clone();
            let event_target = target.clone();
            // let target2 = target.clone();
            
            let _handle = event_target.add_event_listener(move |event: ClickEvent| {
                // let vdom_event = map_event(&handler_type, &event);
                println!("Clicked: {:?} - {:?}", action_target, EventDataTypes::Click);
                // let (_, action) = action_target;
                // model.handle(action_target.payload);
                let action = action_target.payload.clone();
                // (handler)(action);
                // let mut model = model.lock().unwrap();
                // model.handle(action);
                let mut mut_model = arc_model.lock().unwrap();
                mut_model.handle(action);
                let target_vdom = mut_model.render();
                println!("Model: {:?}", mut_model);
                // let vdom = self.vdom.clone();
                let patches = diff(&vdom.clone(), &target_vdom);
                println!("Event Patches: {:?}", patches);
                if let Some (patches) = patches {
                    // target.apply_patches(patches);
                    // let model = self.model.clone();
                    // let vdom = 
                    // let target = self.target.clone();
                    // apply_patches(target, patches, Rc::new(| action | {
                    //     println!("handler action: {:?}", action);
                    //     self.model.handle(action);
                    // }));
                    apply_patches(model.clone(), vdom.clone(), target.clone(), patches);
                }
                // t.handle(action);
            });
            // let bind = self.bind.clone();
            // let action = action.clone();
            // let _handle = target.add_event_listener(enclose!(
            //     (handler_type, action)
            //     move |event: ClickEvent| {
            //         // let action = action.to_owned();
            //         // let handler_type = handler_type.to_owned();
            //         let vdom_event = map_event(&handler_type, &event);
            //         // println!("Clicked: {:?} - {:?} - {:?}", action, handler_type, vdom_event);
            //         println!("Clicked: {:?} - {:?}", action, vdom_event);
            //         //(bind)(action.to_owned(), handler_type.to_owned(), map_event(&handler_type, &event));
            //     }
            // ));
        },

        PatchTypes::Update ( patch_set ) => {
            for i in 0..patch_set.len() {
                let child_nodes: Vec<Node> = target.child_nodes().iter().collect();
                for j in 0..child_nodes.len() {
                    let patches = &patch_set[i];
                    // let child: HtmlElement = child_nodes[j].try_into();
                    // apply_patches(model.clone(), vdom.clone(), child.clone(), patches);
                    apply_patches(model.clone(), vdom.clone(), target.clone(), patches.clone());
                }
            }
            // for patches in patch_set {
            //     apply_patches(model.clone(), vdom.clone(), target.clone(), patches);
            // }
        },
    }
}

#[cfg(test)]
mod tests {
    // use vdom::*;

    #[test]
    fn should_render() {
        // let view: ViewNode =
        //     div(
        //         &[],
        //         &[ text(
        //             format!("{:}", 10)
        //         )],
        //     );
    }
}