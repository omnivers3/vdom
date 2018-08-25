#[macro_use]
extern crate serde_derive;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use std::ops::Deref;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum EventTypes {
    OnClick,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Event<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    pub event_type: EventTypes,
    pub data: TEvents,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum AttributeTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    Attribute(Attribute),
    Event(Event<TEvents>),
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Element<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    pub kind: String,
    pub attributes: Vec<AttributeTypes<TEvents>>,
    pub children: Vec<ContentTypes<TEvents>>,
}

impl<TEvents> Hash for Element<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(&self.kind.as_bytes());
        for attr in &self.attributes {
            state.write_u64(calculate_hash(attr));
        }
        for child in &self.children {
            state.write_u64(calculate_hash(child));
        }
        state.finish();
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum ContentTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    Comment(String),
    Element(Element<TEvents>),
    List(Vec<ContentTypes<TEvents>>),
    Text(String),
}

// impl <TEvents> ContentTypes<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq,
// {
//     fn bind<TTargetEvents, FMap>(self, map: FMap) -> ContentTypes<TEvents>
//     where
//         TTargetEvents: Clone + Eq + Hash + PartialEq,
//         FMap: Fn(TEvents) -> TTargetEvents,
//     {
//         map()
//     }
// }

// pub type StaticAttribute = AttributeTypes<()>;
// pub type StaticElement = ContentTypes<()>;

pub type StaticAttribute = FnAttributeBuilder<()>;
pub type StaticElement = FnContentBuilder<()>;

pub type RenderResult<TEvents> = Option<ContentTypes<TEvents>>;

pub fn attribute<TEvents, TKey, TValue>(key: TKey, value: TValue) -> AttributeTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
    TKey: Into<String>,
    TValue: Into<String>,
{
    AttributeTypes::Attribute(Attribute {
        key: key.into(),
        value: value.into(),
    })
}

pub fn class<TEvents, TClassName>(class_name: TClassName) -> AttributeTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
    TClassName: Into<String>,
{
    AttributeTypes::Attribute(Attribute {
        key: "class".to_owned(),
        value: class_name.into(),
    })
}

pub fn comment<TEvents, T>(value: T) -> ContentTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
    T: Into<String>,
{
    ContentTypes::Comment(value.into())
}

pub fn text<TEvents, T>(value: T) -> ContentTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
    T: Into<String>,
{
    ContentTypes::Text(value.into())
}

pub fn list<TEvents>(children: &[ContentTypes<TEvents>]) -> ContentTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq,
{
    ContentTypes::List(children.to_vec())
}

#[allow(unused)]
macro_rules! class_expand {
    ((), ($($arg:expr),*)) => {{
        [$($arg),*]
    }};

    (($next:expr, $($rest:expr,)*), ($($arg:expr),*)) => {{
        let class: &str = $next;
        class_expand!(($($rest,)*), ($($arg,)* class))
    }};
}

#[allow(unused)]
macro_rules! class {
    ($($class:expr),*) => { class_expand!(($($class,)*), ()) }
}

// TODO: Attributes as a hashset?

pub type FnAttributeBuilder<TEvents>
    where TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
    = fn(IBuildContext<TEvents>) -> AttributeTypes<TEvents>;

pub type FnContentBuilder<TEvents>
    where TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
    = fn(IBuildContext<TEvents>) -> ContentTypes<TEvents>;

pub trait IBuildContext<TEvents>
where
    Self: Sized,
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug
{
    fn element(&self, kind: &str, attributes: &[FnAttributeBuilder<TEvents>], children: &[FnContentBuilder<TEvents>]) -> ContentTypes<TEvents>;
}

macro_rules! element_kind {
    ($kind: ident) => {
        pub fn $kind<TEvents>(
            attributes: &[FnAttributeBuilder<TEvents>],
            children: &[FnContentBuilder<TEvents>],
        ) -> FnContentBuilder<TEvents>
        where
            TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
        {
            &| ctx: &IBuildContext<TEvents> | {
                ctx.element(stringify!(kind), attributes, children)
            }
            // ContentTypes::Element(Element {
            //     kind: stringify!($kind).to_owned(),
            //     attributes: attributes.to_vec(),
            //     children: children.to_vec(),
            // })
        }
    };
}

element_kind!(button);
element_kind!(div);
element_kind!(span);

pub fn on_click<TEvents>(event: TEvents) -> AttributeTypes<TEvents>
where
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
{
    AttributeTypes::Event(Event {
        event_type: EventTypes::OnClick,
        data: event,
    })
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// pub trait IComponent<TEvents>
// where
//     Self: Clone + Default + Eq + Hash + PartialEq + std::fmt::Debug,
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     // fn diff(&self, prev: ContentTypes<TEvents>) -> Option<PatchTypes<TEvents>>;
//     // fn render(&self) -> ContentTypes<TEvents>;
//     // fn handle(&mut self, action: TEvents);
// }

// pub struct Component<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     content: ContentTypes<TEvents>,
// }

// pub trait IComponentProps<TEvents>
// where
//     Self: Clone + Default + Eq + Hash + PartialEq + std::fmt::Debug,
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     fn props<F, FResult>(map: F) -> FResult
//     where
//         F: Fn(&mut Self)
// }

pub trait IComponent<TEvents>
where
    Self: Default,
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
{
    fn render(&self) -> ContentTypes<TEvents>;
}

pub trait IComponentModel<TEvents>
where
    Self: Clone + Default + IComponent<TEvents>,
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
{
    fn props<F>(&self, map: F) -> ContentTypes<TEvents>
    where
        F: Fn(&mut Self) -> ();
}

impl<TModel, TEvents> IComponentModel<TEvents> for TModel
where
    TModel: Clone + Default + IComponent<TEvents>,
    TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
{
    fn props<F>(&self, update: F) -> ContentTypes<TEvents>
    where
        F: Fn(&mut Self) -> (),
    {
        let model: &mut TModel = &mut Self::default();
        update(model);
        model.render()
    }
}

pub enum PatchTypes<TEvents> {
    Event(TEvents),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct Button {
        text: &'static str,
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub enum ButtonEvents {
        Clicked,
    }

    impl Default for Button {
        fn default() -> Self {
            Button { text: "Click Me" }
        }
    }

    impl IComponent<ButtonEvents> for Button {
        fn render(&self) -> ContentTypes<ButtonEvents> {
            button(&[on_click(ButtonEvents::Clicked)], &[text(self.text)])
        }
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct ListView<TItemType> {
        max_items: Option<u64>,
        items: Vec<TItemType>,
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub enum ListViewEvents {
        ItemClicked (u64),
    }

    impl <TItemType> Default for ListView<TItemType> {
        fn default() -> Self {
            ListView {
                max_items: None,
                items: vec![],
            }
        }
    }

    // impl IComponent<ListViewEvents> for ListView<String> {
    //     fn render<TEvents>(&self, events: impl IEventBinding) -> ContentTypes<TEvents>
    //     where
    //         TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
    //     {
    //         let (_, items) = self.items.into_iter().fold((0, vec![]), |(i, list), item| {
    //             i = i + 1;
    //             let item =
    //                 div(
    //                     &[on_click(ListViewEvents::ItemClicked(i))
    //                     ],
    //                     &[ text(item)
    //                     ]);
    //             list.push(item);
    //             (i, list)
    //         });
    //         list(&items)
    //     }
    // }

    pub struct Model {
        list_view: ListView<String>,
        main_button: Button,
    }

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub enum Events {
        ListView(ListViewEvents),
        MainButton(ButtonEvents),
    }

    #[test]
    fn temp() {
        let model = Model {
            list_view: ListView::default(),
            main_button: Button::default(),
        };

        let elem: ContentTypes<Events> = div(
            &[],
            // &[  model.list_view.render(),
            //     model.main_button.props(|m| {
            //         m.text = "Done It"
            //     }),
            //     model.main_button.render()
            // ],
        );
        let elem = list(&[
            elem.to_owned(),
            elem.to_owned(),
        ]);

        println!("{0:?}", elem);

        assert!(false);
    }

    #[test]
    fn static_elem() {
        println!("class: {:?}", class!("blue"));
        println!("class: {:?}", class!("blue", "green"));
        println!("class: {:?}", class!("blue", "value"));
        println!("class: {:?}", class!("blue", "value", "value", "black"));

        assert!(true);
    }

    #[test]
    fn should_hash_attributes() {
        let attr: StaticAttribute = attribute("key", "value");
        let base = calculate_hash(&attr);
        assert_eq!(base, base);

        let attr: StaticAttribute = attribute("key", "value");
        let same = calculate_hash(&attr);
        assert_eq!(base, same);

        let attr: StaticAttribute = attribute("key2", "value");
        let diff_key = calculate_hash(&attr);
        assert!(base != diff_key);

        let attr: StaticAttribute = attribute("key", "value2");
        let diff_value = calculate_hash(&attr);
        assert!(base != diff_value);
    }

    #[test]
    fn should_hash_elements() {
        let base_node: StaticElement = div(&[], &[]);
        let base = calculate_hash(&base_node);
        assert_eq!(base, base);

        let same_node: StaticElement = div(&[], &[]);
        let same = calculate_hash(&same_node);
        assert_eq!(base, same);

        let span_node: StaticElement = span(&[], &[]);
        let span = calculate_hash(&span_node);
        assert!(base != span);
    }

    #[test]
    fn should_build_vdom_tree() {
        let view: StaticElement = div(&[], &[text(format!("{:}", 10))]);

        match view {
            ContentTypes::Element(e) => {
                assert_eq!(e.kind, "div");
                assert_eq!(e.attributes.len(), 0);
                assert_eq!(e.children.len(), 1);

                match e.children[0] {
                    ContentTypes::Text(ref text) => {
                        assert_eq!(text, "10");
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }

    // #[test]
    // fn should_diff_text_elements() {
    //     let delta: Option<PatchTypes<()>> = diff(&Some(text("asdf")), &text("fdsa"));

    //     assert_eq!(delta, Some(PatchTypes::SetText("fdsa".to_owned())));
    // }
}

// pub trait RenderTarget<TEvents> where
//     Self: Clone,
//     TEvents: Clone,
// {
//     fn apply_patches(self, patches: PatchTypes<TEvents>);
// }

// pub fn apply_vdom<TEvents>(
//     target: impl RenderTarget<TEvents>,
//     current_vdom: ContentTypes<TEvents>,
//     target_vdom: ContentTypes<TEvents>
// ) where
//     TEvents: Clone + std::fmt::Debug,
// {
//     let diff_result = diff(current_vdom, target_vdom);
//     println!("Diff: {:?}", diff_result);
//     if let Some (patches) = diff_result {
//         target.apply_patches(patches);
//     }
// }

// fn build<TEvents>(target: &ContentTypes<TEvents>) -> PatchTypes<TEvents>
// where
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     match target {
//         ContentTypes::Empty => PatchTypes::Empty,

//         ContentTypes::Comment(value) => PatchTypes::AddComment(value.to_owned()),

//         ContentTypes::Text(value) => PatchTypes::SetText(value.to_owned()),

//         ContentTypes::Element(element) => PatchTypes::AddElement(
//             element.kind.to_owned(),
//             element
//                 .attributes
//                 .iter()
//                 .map(|attr| (attr.key.to_owned(), attr.value.to_owned()))
//                 .collect(),
//             element.children.iter().map(|child| build(child)).collect(),
//         ),

//         ContentTypes::Handler(handler, action) => {
//             PatchTypes::AddHandler(handler.to_owned(), action.to_owned())
//         }
//     }
// }

// /// TODO: Implement the remove diff
// fn remove<TEvents>() -> PatchTypes<TEvents> where
//     TEvents: Clone + std::fmt::Debug,
// {
//     println!("Remove not implemented");
//     PatchTypes::Empty
// }

// /// TODO: Implement the delta diff
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

// pub fn diff<TEvents>(
//     current: &ContentTypes<TEvents>,
//     desired: &ContentTypes<TEvents>,
// ) -> Option<PatchTypes<TEvents>>
// where
//     TEvents: Clone + Eq + Hash + PartialEq + std::fmt::Debug,
// {
//     match (&current, &desired) {
//         (ContentTypes::Empty, ContentTypes::Empty) => Some(PatchTypes::Empty),

//         (_, ContentTypes::Empty) => Some(PatchTypes::Empty),

//         (ContentTypes::Empty, _) => Some(build(&desired)),

//         (ContentTypes::Comment(prev_value), ContentTypes::Comment(new_value)) => {
//             // TODO: Should replace comment not just add new one
//             Some(PatchTypes::AddComment(new_value.to_owned()))
//         }

//         (ContentTypes::Comment(value), _) => {
//             println!("Replacing node\n{:?}\n\twith comment: {:?}", current, value);
//             Some(PatchTypes::Empty)
//         }

//         (ContentTypes::Text(prev_value), ContentTypes::Text(new_value)) => {
//             if prev_value == new_value {
//                 None
//             } else {
//                 Some(PatchTypes::SetText(new_value.to_owned()))
//             }
//         }

//         (ContentTypes::Text(value), _) => {
//             println!("Replacing node\n{:?}\n\twith text: {:?}", current, value);
//             Some(PatchTypes::Empty)
//         }

//         (ContentTypes::Element(prev_elem), ContentTypes::Element(new_elem)) => {
//             if prev_elem.kind != new_elem.kind {
//                 println!("Swapped element kind");
//                 None
//             } else {
//                 // Same kind
//                 println!(
//                     "Should resolve attrs between\n{:?}\nand\n{:?}\n",
//                     prev_elem.attributes, new_elem.attributes
//                 );
//                 println!(
//                     "Should resolve children between\n{:?}\nand\n{:?}\n",
//                     prev_elem.children, new_elem.children
//                 );
//                 let mut diffs: Vec<PatchTypes<TEvents>> = vec![];
//                 for i in 0..prev_elem.children.len() {
//                     let prev_child = &prev_elem.children[i];
//                     for j in i..new_elem.children.len() {
//                         let new_child = &new_elem.children[j];
//                         if let Some(diff) = diff(prev_child, new_child) {
//                             println!(
//                                 "Diffed children\n{:?}\nand\n{:?}\n{:?}\n",
//                                 prev_child, new_child, diff
//                             );
//                             diffs.push(diff);
//                         }
//                     }
//                 }
//                 Some(PatchTypes::Update(diffs))
//                 // for child in prev_elem.children {
//                 //     let diff = diff(child)
//                 // }
//             }
//             // None
//         }

//         (_, _) => {
//             println!(
//                 "Diff not yet implemented between\n{:?}\nand\n{:?}\n",
//                 current, desired
//             );
//             None
//         }
//     }
// }

// macro_rules! attribute_expand {
//     ((), ($($arg:expr),*)) => {{
//         ([$($arg),*])
//     }};

//     (($next:expr, $($rest:expr,)*), ($($arg:expr),*)) => {{
//         let attribute: &str = $next;
//         attribute_expand!(($($rest,)*), ($($arg,)* attribute))
//     }};
// }

// macro_rules! attribute {
//     ($($attribute:expr),*) => { attribute_expand!(($($attribute,)*), ()) }
// }

// macro_rules! element {
//     ($kind: ident) => {
//         macro_rules! $kind {
//             ([$attributes:expr], [$children:expr]) => {
//                 {
//                     println!(stringify!($kind));
//                 }
//                 // ( stringify!($kind), attribute!($attributes), attribute!($children) )
//             };
//             // ($($attributes:expr),*,$($children:expr),*) => { class_expand!(($($attributes,)*), (), ($($children,)*)) }
//             // ($data: expr) => {
//             //     pub fn $kind() {
//             //         println!("{:?}: {:?}", stringify!($kind), $data);
//             //     }
//             // }
//             (($attributes:expr), ($children:expr),) => {
//                 kind!($attributes, $children)
//                 // ( stringify!($kind), attribute!($attributes), attribute!($children) )
//             };
//         }
//     };
// }

// element!(blah);

// blah!(["stuff"], ["asdf"]);

// element_kind_fn!(div);
// element_kind_fn!(span);
// element_kind_fn!(button);

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub enum EventDataTypes {
//     Click,
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub enum HandlerTypes {
//     OnClick,
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub enum PatchTypes<TEvents>
// where
//     TEvents: Clone + std::fmt::Debug,
// {
//     SetText(String),
//     AddElement(String, Vec<(String, String)>, Vec<PatchTypes<TEvents>>),
//     AddHandler(HandlerTypes, ActionTarget<TEvents>),
//     Update(Vec<PatchTypes<TEvents>>),
// }

// fn s_element(kind: &'static str, attributes: &[&'static str], children: &[&'static str]) {
//     println!("Element: {:?}", kind);
// }

// fn data(v: u64) -> String {
//     if v < 1000 {
//         "foo".to_owned()
//     } else {
//         "bar".to_owned()
//     }
// }

// fn data2<'a>(v: u64) -> &'a str {
//     if v < 1000 {
//         "foo"
//     } else {
//         "bar"
//     }
// }

// fn data3(v: String) -> &'static str {
//     if v == "asdf" {
//         "foo"
//     } else {
//         "bar"
//     }
// }
