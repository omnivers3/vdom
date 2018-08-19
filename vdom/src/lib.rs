#[macro_use]
extern crate serde_derive;

use std::collections::{ HashMap };
use std::collections::hash_map::{ DefaultHasher };
use std::hash::{Hash, Hasher};
// use std::ops::Deref;
// use std::rc::{ Rc, Weak };
// use std::rc::{ Weak };

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum EventTypes {
    OnClick,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum AttributeTypes<TEvents> 
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    Event(EventTypes, TEvents),
    Attribute(String, String),
}

// #[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct Element//<TEvents>
// // where
// //     TEvents: Clone + Eq + Hash + PartialEq,
// {
//     // ctx: &Context<TEvents>,
// }

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ContentTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    Element(String),
    Event(TEvents),
    Text(String),
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Node {
    kind: u64,
    attributes: Vec<u64>,
    children: Vec<u64>,
}

impl Hash for Node {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        // state.write(&self.kind.as_bytes());
        state.write_u64(self.kind);
        for attr in &self.attributes {
            state.write_u64(*attr);
        }
        for child in &self.children {
            state.write_u64(*child);
        }
        state.finish();
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Elements<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    Element (String),
    Event (TEvents)
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct Context<TEvents, S = RandomState>
pub struct Context<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    strings: HashMap<u64, String>,
    elements: HashMap<u64, Elements<TEvents>>,
}

impl <TEvents> Context<TEvents> 
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    pub fn new() -> Self {
        Context {
            strings: HashMap::new(),
            elements: HashMap::new(),
        }
    }

    pub fn strings(&self) -> HashMap<u64, String> {
        self.strings.to_owned()
    }

    pub fn element<TKey: Into<String>>(&mut self, key: TKey) -> u64 {
        let key = key.into();
        let hash = calculate_hash(&key);
        if let Some(_) = self.strings.insert(hash, key.to_owned()) {
            panic!("Hash collision")
        }
        hash
    }
}

pub type ViewAttribute = AttributeTypes<()>;
pub type ViewNode = ContentTypes<()>;

pub type RenderResult<TEvents> = Option<ContentTypes<TEvents>>;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[macro_export]
macro_rules! rsx {
    ($kind: ident) => {
        stringify!($kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hash_attributes() {
        let mut ctx: Context<()> = Context::new();

        let hash = ctx.element("div");

        // assert_eq!(ctx.strings().iter().collect()[0].1, "div");
        // println!("{:?}", &ctx.strings()[&hash]);
        assert_eq!("div", ctx.strings()[&hash]);

        // let attr: ViewAttribute = attribute("key", "value");
        // let base = calculate_hash(&attr);
        // assert_eq!(base, base);

        // let attr: ViewAttribute = attribute("key", "value");
        // let same = calculate_hash(&attr);
        // assert_eq!(base, same);

        // let attr: ViewAttribute = attribute("key2", "value");
        // let diff_key = calculate_hash(&attr);
        // assert!(base != diff_key);

        // let attr: ViewAttribute = attribute("key", "value2");
        // let diff_value = calculate_hash(&attr);
        // assert!(base != diff_value);
    }

    #[test]
    fn should_hash_elements() {
        // let base_node: ContentTypes<()> = div(&[], &[]);
        // let base = calculate_hash(&base_node);
        // assert_eq!(base, base);
        // // match base_node {
        // //     ContentTypes::Element (e) => assert_eq!(e.uuid, e.uuid),
        // //     _ => assert!(false),
        // // }

        // let same_node: ContentTypes<()> = div(&[], &[]);
        // let same = calculate_hash(&same_node);
        // assert_eq!(base, same);
        // // match (base_node, same_node) {
        // //     (ContentTypes::Element (base), ContentTypes::Element (same)) => assert_eq!(base.uuid, same.uuid),
        // //     (_, _) => assert!(false),
        // // }

        // let span_node: ContentTypes<()> = span(&[], &[]);
        // let span = calculate_hash(&span_node);
        // assert!(base != span);
    }

    #[test]
    fn should_build_vdom_tree() {
        // let view = rsx!(
        //     div
        // );
        // assert_eq!(view, "div");
        // let view = rsx!(
        //     div
        //         []
        //         [ text format!("{:}". 10)
        //         ]
        // );
        // let view: ViewNode = div(&[], &[text(format!("{:}", 10))]);

        // match view {
        //     ContentTypes::Element(e) => {
        //         assert_eq!(e.kind, "div");
        //         assert_eq!(e.attributes.len(), 0);
        //         assert_eq!(e.children.len(), 1);

        //         match e.children[0] {
        //             ContentTypes::Text(ref text) => {
        //                 assert_eq!(text, "10");
        //             }
        //             _ => assert!(false),
        //         }
        //     }
        //     _ => assert!(false),
        // }
    }

    // #[test]
    // fn should_diff_text_elements() {
    //     let delta: Option<PatchTypes<()>> = diff(&Some(text("asdf")), &text("fdsa"));

    //     assert_eq!(delta, Some(PatchTypes::SetText("fdsa".to_owned())));
    // }
}


// pub fn attribute<TEvents, TKey, TValue>(key: TKey, value: TValue) -> AttributeTypes<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
//     TKey: Into<String>,
//     TValue: Into<String>,
// {
//     AttributeTypes::Attribute(key.into(), value.into())
// }

// pub fn class<TEvents, TString>(class_name: TString) -> AttributeTypes<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
//     TString: Into<String>,
// {
//     AttributeTypes::Attribute("class".to_owned(), class_name.into())
// }

// macro_rules! action_kind {
//     ($kind: ident, $type: expr) => {
//         pub fn $kind<TEvents>(payload: TEvents) -> AttributeTypes<TEvents>
//         where
//             TEvents: Clone + Eq + Hash + PartialEq,
//         {
//             AttributeTypes::Action($type, payload)
//         }
//     };
// }

// action_kind!(on_click, EventTypes::OnClick);

// pub fn text<TEvents, T>(value: T) -> ContentTypes<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
//     T: Into<String>,
// {
//     ContentTypes::text(value)
// }

// macro_rules! element_kind {
//     ($kind: ident) => {
//         pub fn $kind<TEvents>(
//             attributes: &[AttributeTypes<TEvents>],
//             children: &[ContentTypes<TEvents>],
//         ) -> ContentTypes<TEvents>
//         where
//             TEvents: Clone + Eq + Hash + PartialEq,
//         {
//             ContentTypes::<TEvents>::element(stringify!($kind), attributes, children)
//         }
//     };
// }

// element_kind!(div);
// element_kind!(span);
// element_kind!(button);

// #[macro_export]
// macro_rules! on_click {
//     ($command: expr) => {
//         let target = ActionTarget::with_payload(HandlerTypes::OnClick, $command);
//         ContentTypes::Handler(target)
//     };
// }

// pub trait Component<TEvents>
// where
//     Self: Clone + Default + std::fmt::Debug,
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     fn render(&self) -> ContentTypes<TEvents>;
//     fn handle(&mut self, action: TEvents);
// }


// impl<TEvents> ContentTypes<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
// {
//     pub fn element<TKind, TAttributes, TChildren>(
//         kind: TKind,
//         attributes: TAttributes,
//         children: TChildren,
//     ) -> Self
//     where
//         TKind: Into<String>,
//         TAttributes: Deref<Target = [AttributeTypes<TEvents>]>,
//         TChildren: Deref<Target = [ContentTypes<TEvents>]>,
//     {
//         let kind: String = kind.into();
//         let attributes: Vec<AttributeTypes<TEvents>> = attributes.deref().to_vec();
//         let children: Vec<ContentTypes<TEvents>> = children.deref().to_vec();

//         let element = Element::new(kind, attributes, children);
//         ContentTypes::Element(element)
//     }

//     pub fn text<T>(value: T) -> Self
//     where
//         T: Into<String>,
//     {
//         let value: String = value.into();
//         ContentTypes::Text(value)
//     }
// }


// #[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct Element<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
// {
//     pub kind: u64,
//     pub attributes: Vec<u64>,
//     pub children: Vec<u64>,
//     // pub attributes: Vec<AttributeTypes<TEvents>>,
//     // pub children: Vec<ContentTypes<TEvents>>,
// }


/*
fn build<TEvents>(target: &ContentTypes<TEvents>) -> PatchTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
{
    match target {
        ContentTypes::Comment(node) => PatchTypes::AddComment(node.item.to_owned()),

        ContentTypes::Text(node) => PatchTypes::SetText(node.item.to_owned()),

        ContentTypes::Element(node) => {
            let element = &node.item;
            PatchTypes::AddElement(
                element.kind.to_owned(),
                element
                    .attributes
                    .iter()
                    .map(|attr| (attr.key.to_owned(), attr.value.to_owned()))
                    .collect(),
                element.children.iter().map(|child| build(child)).collect(),
            )
        }

        ContentTypes::Handler(node) => {
            let action_target = &node.item;
            PatchTypes::AddHandler(action_target.to_owned())
            //     ActionTarget::with_payload(
            //         action_target.type.to_owned(),
            //         action_target.payload.to_owned(),
            //     )
            // )
        }
    }
}

pub fn diff<TEvents>(
    current: &Option<ContentTypes<TEvents>>,
    desired: &ContentTypes<TEvents>,
) -> Option<PatchTypes<TEvents>>
where
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
{
    match (&current, &desired) {
        (None, _) => Some(build(&desired)),
        (Some(current), _) => None,
    }
}
*/


// pub attributes: Vec<AttributeTypes<TEvents>>,
// pub children: Vec<ContentTypes<TEvents>>,

// impl<TEvents> Element<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
// {
//     fn new(
//         kind: String,
//         attributes: Vec<AttributeTypes<TEvents>>,
//         children: Vec<ContentTypes<TEvents>>,
//     ) -> Self {
//         Element {
//             kind,
//             attributes,
//             children,
//         }
//     }
// }


// #[derive(Eq, Hash, PartialEq)]
// #[derive(Clone, Debug)]
// #[derive(Serialize, Deserialize)]
// pub enum EventDataTypes {
//     Click,
// }

// #[derive(Eq, Hash, PartialEq)]
// #[derive(Clone, Debug)]
// #[derive(Serialize, Deserialize)]
// pub enum PatchTypes<TEvents> where
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     SetText ( String ),
//     AddElement ( String, Vec<(String, String)>, Vec<PatchTypes<TEvents>> ),
//     // AddHandler ( ActionTarget<TEvents> ),
//     Update ( Vec<PatchTypes<TEvents>> ),
// }

// /// TODO: Implement the remove diff
// fn remove<TEvents>() -> PatchTypes<TEvents> where
//     TEvents: Clone + std::fmt::Debug,
// {
//     println!("Remove not implemented");
//     PatchTypes::Empty
// }

// TODO: Implement the delta diff
// fn delta<TEvents>(
//     current: ContentTypes<TEvents>,
//     desired: ContentTypes<TEvents>
// ) -> Option<PatchTypes<TEvents>> where
//     TEvents: Clone + std::fmt::Debug,
// {
//     println!("Delta not implemented");
//     // None
//     match ( &current, &desired ) {
//         ( ContentTypes::)
//         ( ContentTypes::Empty, ContentTypes::Empty ) => None,

//         ( _, ContentTypes::Empty ) => Some ( remove() ),

//         ( ContentTypes::Empty, _ ) => Some ( build(&desired) ),

//         ( _, _ ) => {
//             // delta(current, desired),
//             println!("Delta [\n{:?}\n -> \n{:?}\n]", current, desired);
//             None
//         },
//     }
// }

/*
{
            match ( current, desired ) {

                ( ContentTypes::Comment (prev_value), ContentTypes::Comment (new_value) ) => {
                    if prev_value == new_value {
                        None
                    } else {
                        // TODO: Should replace comment not just add new one
                        Some ( PatchTypes::AddComment (new_value.to_owned()) )
                    }
                },

                ( ContentTypes::Comment (value), _ ) => {
                    println!("Replacing node\n{:?}\n\twith comment: {:?}", current, value);
                    Some ( PatchTypes::Empty )
                },

                ( ContentTypes::Text (prev_value), ContentTypes::Text (new_value) ) => {
                    if prev_value == new_value {
                        None
                    } else {
                        Some ( PatchTypes::SetText (new_value.to_owned()) )
                    }
                },

                ( ContentTypes::Text (value), _ ) => {
                    println!("Replacing node\n{:?}\n\twith text: {:?}", current, value);
                    Some ( PatchTypes::Empty )
                },

                ( ContentTypes::Element (prev_elem), ContentTypes::Element (new_elem) ) => {
                    if prev_elem.kind != new_elem.kind {
                        println!("Swapped element kind");
                        None
                    } else { // Same kind
                        println!("Should resolve attrs between\n{:?}\nand\n{:?}\n", prev_elem.attributes, new_elem.attributes);
                        println!("Should resolve children between\n{:?}\nand\n{:?}\n", prev_elem.children, new_elem.children);
                        let mut diffs: Vec<PatchTypes<TEvents>> = vec![];
                        for i in 0..prev_elem.children.len() {
                            let prev_child = &prev_elem.children[i];
                            for j in i..new_elem.children.len() {
                                let new_child = &new_elem.children[j];
                                if let Some (diff) = diff(prev_child, new_child) {
                                    println!("Diffed children\n{:?}\nand\n{:?}\n{:?}\n", prev_child, new_child, diff);
                                    diffs.push(diff);
                                }
                                
                            }
                        }
                        Some ( PatchTypes::Update (diffs) )
                        // for child in prev_elem.children {
                        //     let diff = diff(child)
                        // }
                    }
                    // None
                },

                ( _, _ ) => {
                    println!("Diff not yet implemented between\n{:?}\nand\n{:?}\n", current, desired);
                    None
                },

            }
        }
    }
}
*/
