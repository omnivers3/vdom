use std::ops::Deref;

#[macro_use]
extern crate serde_derive;

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct ActionTarget<TActions> where
    TActions: Clone,
{
    pub payload: TActions,
}

impl <TActions> ActionTarget<TActions> where
    TActions: Clone,
{
    pub fn with_payload(payload: TActions) -> Self {
        ActionTarget {
            payload,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Element<TActions> where
    TActions: Clone,
{
    pub kind: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<NodeTypes<TActions>>,
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum EventDataTypes {
    Click,
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum HandlerTypes {
    OnClick,
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum PatchTypes<TActions> where
    TActions: Clone + std::fmt::Debug,
{
    Empty,
    AddComment ( String ),
    SetText ( String ),
    AddElement ( String, Vec<(String, String)>, Vec<PatchTypes<TActions>> ),
    AddHandler ( HandlerTypes, ActionTarget<TActions> ),
    Update ( Vec<PatchTypes<TActions>> ),
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum NodeTypes<TActions> where
    TActions: Clone,
{
    Empty,
    Comment ( String ),
    Text ( String ),
    Element ( Element<TActions> ),
    Handler ( HandlerTypes, ActionTarget<TActions> ),
}

impl <TActions> NodeTypes<TActions> where
    TActions: Clone,
{
    pub fn comment<T>(value: T) -> Self where
        T: Into<String>,
    {
        NodeTypes::Comment(
            value.into()
        )
    }

    /// Makes an instance of a Virtual Element with properties set to provided values
    pub fn element<TKind, TAttributes, TChildren>(
        kind: TKind,
        attributes: TAttributes,
        children: TChildren
    ) -> Self where
        TKind: Into<String>,
        TAttributes: Deref<Target=[Attribute]>,
        TChildren: Deref<Target=[NodeTypes<TActions>]>,
    {
        let kind_: String = kind.into();
        let attributes_: Vec<Attribute> = attributes.deref().to_vec();
        let children_: Vec<NodeTypes<TActions>> = children.deref().to_vec();
        NodeTypes::Element (
            Element {
                kind: kind_,
                attributes: attributes_,
                children: children_,
            }
        )
    }

    pub fn text<T>(value: T) -> Self where
        T: Into<String>,
    {
        NodeTypes::Text (
            value.into()
        )
    }
}

pub type ViewNode = NodeTypes<()>;

pub type RenderResult<TActions> = Option<NodeTypes<TActions>>;

pub fn attribute<TKey, TValue>(key: TKey, value: TValue) -> Attribute where
    TKey: Into<String>,
    TValue: Into<String>,
{
    Attribute {
        key: key.into(),
        value: value.into(),
    }
}

pub fn class<T>(class_name: T) -> Attribute where
    T: Into<String>
{
    Attribute {
        key: "class".to_string(),
        value: class_name.into(),
    }
}

pub fn text<TActions, T>(value: T) -> NodeTypes<TActions> where
    TActions: Clone,
    T: Into<String>
{
    NodeTypes::text(value)
}

macro_rules! element_kind {
    ($kind: ident) => {
        pub fn $kind<TActions>(
            attributes: &[Attribute],
            children: &[NodeTypes<TActions>]
        ) -> NodeTypes<TActions> where
            TActions: Clone,
        {
            NodeTypes::<TActions>::element(stringify!($kind), attributes, children)
        }
    }
}

element_kind!(div);
element_kind!(span);
element_kind!(button);

#[macro_export]
macro_rules! on_click {
    ($command: expr) => {
        NodeTypes::Handler (
            HandlerTypes::OnClick,
            ActionTarget::with_payload($command),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_vdom_tree() {
        let view: ViewNode =
            div(
                &[],
                &[ text(
                    format!("{:}", 10)
                )],
            );
        
        match view {
            NodeTypes::Element(e) => {
                assert_eq!(e.kind, "div");
                assert_eq!(e.attributes.len(), 0);
                assert_eq!(e.children.len(), 1);

                match e.children[0] {
                    NodeTypes::Text(ref text) => {
                        assert_eq!(text, "10");
                    },
                    _ => assert!(false)
                }
            }
            _ => assert!(false)
        }
    }
}

// pub trait RenderTarget<TActions> where
//     Self: Clone,
//     TActions: Clone,
// {
//     fn apply_patches(self, patches: PatchTypes<TActions>);
// }

pub trait Component<TActions> where
    Self: Clone + Default + std::fmt::Debug,
    TActions: Clone + std::fmt::Debug,
{
    fn render(&self) -> NodeTypes<TActions>;
    fn handle(&mut self, action: TActions);
}

// pub fn apply_vdom<TActions>(
//     target: impl RenderTarget<TActions>,
//     current_vdom: NodeTypes<TActions>,
//     target_vdom: NodeTypes<TActions>
// ) where
//     TActions: Clone + std::fmt::Debug,
// {
//     let diff_result = diff(current_vdom, target_vdom);
//     println!("Diff: {:?}", diff_result);
//     if let Some (patches) = diff_result {
//         target.apply_patches(patches);
//     }
// }

fn build<TActions>(
    target: &NodeTypes<TActions>
) -> PatchTypes<TActions> where
    TActions: Clone + std::fmt::Debug,
{
    
    match target {

        NodeTypes::Empty => PatchTypes::Empty,

        NodeTypes::Comment ( value ) => PatchTypes::AddComment ( value.to_owned() ),

        NodeTypes::Text ( value ) => PatchTypes::SetText ( value.to_owned() ),

        NodeTypes::Element ( element ) => PatchTypes::AddElement (
            element.kind.to_owned(),
            element.attributes.iter().map(|attr| ( attr.key.to_owned(), attr.value.to_owned())).collect(),
            element.children.iter().map(|child| {
                build(child)
            }).collect(),
        ),

        NodeTypes::Handler ( handler, action ) => {
            PatchTypes::AddHandler (
                handler.to_owned(),
                action.to_owned(),
            )
        }
    }

}

// /// TODO: Implement the remove diff
// fn remove<TActions>() -> PatchTypes<TActions> where
//     TActions: Clone + std::fmt::Debug,
// {
//     println!("Remove not implemented");
//     PatchTypes::Empty
// }

/// TODO: Implement the delta diff
// fn delta<TActions>(
//     current: NodeTypes<TActions>,
//     desired: NodeTypes<TActions>
// ) -> Option<PatchTypes<TActions>> where
//     TActions: Clone + std::fmt::Debug,
// {
//     println!("Delta not implemented");
//     // None
//     match ( &current, &desired ) {
//         ( NodeTypes::)
//         ( NodeTypes::Empty, NodeTypes::Empty ) => None,

//         ( _, NodeTypes::Empty ) => Some ( remove() ),

//         ( NodeTypes::Empty, _ ) => Some ( build(&desired) ),

//         ( _, _ ) => {
//             // delta(current, desired),
//             println!("Delta [\n{:?}\n -> \n{:?}\n]", current, desired);
//             None
//         },
//     }
// }

pub fn diff<TActions>(
    current: &NodeTypes<TActions>,
    desired: &NodeTypes<TActions>
) -> Option<PatchTypes<TActions>> where
    TActions: Clone + std::fmt::Debug,
{
    match ( &current, &desired ) {
        ( NodeTypes::Empty, NodeTypes::Empty ) => {
            Some ( PatchTypes::Empty )
        },

        ( _, NodeTypes::Empty ) => {
            Some ( PatchTypes::Empty )
        },

        ( NodeTypes::Empty, _ ) => {
            Some ( build(&desired) )
        },

        ( NodeTypes::Comment (prev_value), NodeTypes::Comment (new_value) ) => {
            // TODO: Should replace comment not just add new one
            Some ( PatchTypes::AddComment (new_value.to_owned()) )
        },

        ( NodeTypes::Comment (value), _ ) => {
            println!("Replacing node\n{:?}\n\twith comment: {:?}", current, value);
            Some ( PatchTypes::Empty )
        },

        ( NodeTypes::Text (prev_value), NodeTypes::Text (new_value) ) => {
            if prev_value == new_value {
                None
            } else {
                Some ( PatchTypes::SetText (new_value.to_owned()) )
            }
        },

        ( NodeTypes::Text (value), _ ) => {
            println!("Replacing node\n{:?}\n\twith text: {:?}", current, value);
            Some ( PatchTypes::Empty )
        },

        ( NodeTypes::Element (prev_elem), NodeTypes::Element (new_elem) ) => {
            if prev_elem.kind != new_elem.kind {
                println!("Swapped element kind");
                None
            } else { // Same kind
                println!("Should resolve attrs between\n{:?}\nand\n{:?}\n", prev_elem.attributes, new_elem.attributes);
                println!("Should resolve children between\n{:?}\nand\n{:?}\n", prev_elem.children, new_elem.children);
                let mut diffs: Vec<PatchTypes<TActions>> = vec![];
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
