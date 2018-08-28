#[macro_use]
extern crate serde_derive;

use std::marker::{ PhantomData };
use std::collections::hash_map::DefaultHasher;
use std::fmt::{ Debug };
use std::hash::{Hash, Hasher};

/// Define the expected global trait set for events
pub trait IEvent: Eq + Hash + PartialEq + Debug {}

/// Auto-implement IEvent for types which share the required traits
impl<TTarget> IEvent for TTarget
where
    TTarget: Eq + Hash + PartialEq + Debug,
{}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct OnClickEvent {}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum EventTypes {
    OnClick (OnClickEvent),
}

impl From<OnClickEvent> for EventTypes {
    fn from(src: OnClickEvent) -> EventTypes {
        EventTypes::OnClick (src)
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Event<TEvents>
where
    TEvents: IEvent,
{
    pub event_type: EventTypes,
    pub data: TEvents,
}

impl <TEvents> Event<TEvents>
where
    TEvents: IEvent,
{
    pub fn map<TWrappingEvents, FnWrapper>(
        self,
        wrapper: FnWrapper,
    ) -> Event<TWrappingEvents>
    where
        TWrappingEvents: IEvent,
        FnWrapper: FnOnce(TEvents) -> TWrappingEvents + Clone,
    {
        Event {
            event_type: self.event_type,
            data: wrapper(self.data),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node<TDatum> {
    pub datum: TDatum,
    pub children: Vec<Node<TDatum>>,
}

#[derive(Clone, Debug)]
pub struct CustomAttribute {
    pub key: String,
    pub value: String,
}

impl<TEvents> IAttribute<TEvents> for CustomAttribute where TEvents: IEvent {}

#[derive(Clone, Debug)]
pub struct ClassAttribute {
    pub values: Vec<String>,
}

impl<TEvents> IAttribute<TEvents> for ClassAttribute where TEvents: IEvent {}

#[derive(Clone, Debug)]
pub struct EventAttribute<TEvents>
where
    TEvents: IEvent,
{
    event: Event<TEvents>,
}

impl<TEvents> IAttribute<TEvents> for EventAttribute<TEvents> where TEvents: IEvent {}

#[derive(Clone, Debug)]
pub enum AttributeTypes<TEvents>
where
    TEvents: IEvent,
{
    Custom (CustomAttribute),
    Class (ClassAttribute),
    Event (EventAttribute<TEvents>),
}

impl<TEvents> From<CustomAttribute> for AttributeTypes<TEvents>
where
    TEvents: IEvent,
{
    fn from(src: CustomAttribute) -> AttributeTypes<TEvents> {
        AttributeTypes::Custom (src)
    }
}

impl<TEvents> From<ClassAttribute> for AttributeTypes<TEvents>
where
    TEvents: IEvent,
{
    fn from(src: ClassAttribute) -> AttributeTypes<TEvents> {
        AttributeTypes::Class (src)
    }
}

impl<TEvents> From<EventAttribute<TEvents>> for AttributeTypes<TEvents>
where
    TEvents: IEvent,
{
    fn from(src: EventAttribute<TEvents>) -> AttributeTypes<TEvents> {
        AttributeTypes::Event (src)
    }
}

#[derive(Clone, Debug)]
pub struct CustomElement {
    pub kind: String,
}

#[derive(Clone, Debug)]
pub struct HtmlElement {}

#[derive(Clone, Debug)]
pub struct BodyElement {}

#[derive(Clone, Debug)]
pub enum ElementTypes {
    Custom (CustomElement),
    Html (HtmlElement),
    Body (BodyElement),
}

impl From<CustomElement> for ElementTypes {
    fn from(src: CustomElement) -> ElementTypes {
        ElementTypes::Custom (src)
    }
}

impl From<HtmlElement> for ElementTypes {
    fn from(src: HtmlElement) -> ElementTypes {
        ElementTypes::Html (src)
    }
}

impl From<BodyElement> for ElementTypes {
    fn from(src: BodyElement) -> ElementTypes {
        ElementTypes::Body (src)
    }
}

#[derive(Clone, Debug)]
pub struct ElementDatum<TEvents>
where
    TEvents: IEvent,
{
    pub element: ElementTypes,
    pub attributes: Vec<AttributeTypes<TEvents>>,
}

pub type Element<TEvents> = Node<ElementDatum<TEvents>>;

impl<TEvents> From<Node<ElementDatum<TEvents>>> for ElementTypes
where
    TEvents: IEvent,
{
    fn from(src: Node<ElementDatum<TEvents>>) -> ElementTypes {
        src.into()
    }
}
impl<TEvents> IElement<TEvents> for Element<TEvents> where TEvents: IEvent {}

impl<TEvents> Element<TEvents>
where
    TEvents: IEvent,
{
    pub fn new(datum: ElementDatum<TEvents>, children: Vec<Node<ElementDatum<TEvents>>>) -> Self {
        Node::<ElementDatum<TEvents>> {
            datum,
            children,
        }
    }
}

pub trait IAttribute<TEvents>: Into<AttributeTypes<TEvents>> where TEvents: IEvent {}

// pub trait IElement<TEvents>: Into<ElementTypes> where TEvents: IEvent {}
pub trait IElement<TEvents>: Into<Element<TEvents>> where TEvents: IEvent {}

// pub fn element<TEvents, TAttributes, TChildren>(kind: &str, attributes: &[TAttributes], children: &[TChildren]) -> Element<TEvents>
// where
//     TEvents: IEvent + Clone,
//     TAttributes: IAttribute<TEvents> + Clone,
//     TChildren: IElement<TEvents> + Clone,
// {
//     let element = CustomElement { kind: kind.to_owned() }.into();
//     let attributes = attributes.into_iter().map(|item| (*item).to_owned().into()).collect();
//     let children = children.into_iter().map(|item| {
//         let temp: Element<TEvents> = (*item).to_owned().into();
//         temp
//     }).collect();
//     // let children = children.to_vec();

//     Element::<TEvents>::new(
//         ElementDatum::<TEvents> {
//             element,
//             attributes,
//         },
//         children,
//     )
// }

#[derive(Clone, Debug)]
pub struct Empty {}

pub struct Html<TEvents, TAttributes=CustomAttribute, TChildren=Empty>
where
    TEvents: IEvent + Clone,
    TAttributes: IAttribute<TEvents> + Clone,
    TChildren: IElement<TEvents> + Clone,
{
    events: PhantomData<TEvents>,
    attributes: PhantomData<TAttributes>,
    children: PhantomData<TChildren>,
}

impl<TEvents, TAttributes, TChildren> Html<TEvents, TAttributes, TChildren>
where
    TEvents: IEvent + Clone,
    TAttributes: IAttribute<TEvents> + Clone,
    TChildren: IElement<TEvents> + Clone,
{
    pub fn element(kind: &str, attributes: &[TAttributes], children: &[TChildren]) -> Element<TEvents>
    {
        let element = CustomElement { kind: kind.to_owned() }.into();
        let attributes = attributes.into_iter().map(|item| (*item).to_owned().into()).collect();
        let children = children.into_iter().map(|item| {
            let temp: Element<TEvents> = (*item).to_owned().into();
            temp
        }).collect();
        // let children = children.to_vec();

        Element::<TEvents>::new(
            ElementDatum::<TEvents> {
                element,
                attributes,
            },
            children,
        )
    }
}

// pub fn element<TEvents>(kind: &str, attributes: &[AttributeTypes<TEvents>], children: &[Element<TEvents>]) -> Element<TEvents>
// where
//     TEvents: IEvent + Clone,
// {
//     let element = CustomElement { kind: kind.to_owned() }.into();
//     let attributes = attributes.to_vec();
//     let children = children.to_vec();
//     Element::<TEvents>::new(
//         ElementDatum::<TEvents> {
//             element,
//             attributes,
//         },
//         children,
//     )
// }

pub fn class(values: &[&str]) -> ClassAttribute {
    let values = values.into_iter().map(|item| (*item).to_owned()).collect();
    ClassAttribute {
        values,
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compose() {
        let e: Element<()> = Html::element("asdf",
            &[ ClassAttribute { values: vec!["blue_button".to_owned()] }.into()
            , class(&["green_button", "black_button"])
            ],
            &[ Html::element("blah", &[], &[])

            ]);

        println!("{0:?}", e);
        // let html1 = html(&[], &[]);
        // let html2 = html(&[], &[html1.clone()]);

        // println!("{0:?}", html1);

        assert!(false);
    }

    // #[test]
    // fn should_element() {
    //     let element1 = Element::<()>::new("element-a", &[], &[]);
    //     let element2 = Element::new("element-a", &[], &[element1.clone()]);
    //     let element3 = Element::<()>::new("element-a", &[], &[Element::new("element-b", &[], &[])]);

    //     println!("{0:?}", element1);
    //     println!("{0:?}", element2);
    //     println!("{0:?}", element3);

    //     assert!(true);
    // }
}

// pub trait IHtmlAttribute: std::fmt::Debug {}

// impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

// pub trait IHtmlChild: std::fmt::Debug {}

// impl<'a> IHtmlChild for &'a IHtmlChild {}

// #[derive(Clone, Debug)]
// pub enum HtmlAttributes {
// }

// #[derive(Clone, Debug)]
// pub enum HtmlChildren {
//     Body (BodyChildren),
// }

// #[derive(Clone, Debug)]
// pub enum BodyAttributes {

// }

// #[derive(Clone, Debug)]
// pub enum BodyChildren {

// }

// impl From<BodyChildren> for HtmlChildren {
//     fn from(source: BodyChildren) -> HtmlChildren {
//         HtmlChildren::Body (source)
//     }
// }

// #[derive(Clone, Debug)]
// pub struct HtmlElement<TAttributes>(Element<TAttributes>);

// pub fn html<TAttributes, TChildren>(
//     attributes: &[TAttributes],
//     children: &[TChildren]
// ) -> HtmlElement<TAttributes>
// where
//     TAttributes: Into<HtmlAttributes> + Clone + std::fmt::Debug,
//     TChildren: Clone + std::fmt::Debug,
// {
//     let children = children
//         .into_iter()
//         .map(|item| {
//             let out: Element<TAttributes> = (*item).into();
//             out
//         })
//         .collect()
//         .to_vec();
//     HtmlElement::<TAttributes>(
//         Element::new("html", attributes, children)
//     )
// }

// pub trait IElement {
//     type TAttributes;
// }

// pub trait IAttribute: std::fmt::Debug {}

// impl<'a> IAttribute for &'a IAttribute {}

// pub trait IElement: std::fmt::Debug {}

// impl<'a> IElement for &'a IElement {}

// #[derive(Clone, Debug)]
// pub struct Element<'a, TAttributes: 'a, TChildren: 'a>
// where
//     TAttributes: IAttribute + Clone,
//     &'a IAttribute: From<TAttributes>,
//     TChildren: IElement + Clone,
//     &'a IElement: From<TChildren>,
// {
//     pub kind: &'a str,
//     pub attributes: &'a [TAttributes],
//     pub children: &'a [TChildren],
// }

// impl<'a, TAttributes: 'a, TChildren: 'a> IElement for Element<'a, TAttributes, TChildren>
// where
//     TAttributes: IAttribute + Clone,
//     &'a IAttribute: From<TAttributes>,
//     TChildren: IElement + Clone,
//     &'a IElement: From<TChildren>,
// {}

// impl<'a, TAttributes: 'a, TChildren: 'a> Element<'a, TAttributes, TChildren>
// where
//     TAttributes: IAttribute + Clone,
//     &'a IAttribute: From<TAttributes>,
//     TChildren: IElement + Clone,
//     &'a IElement: From<TChildren>,
// {
//     fn new(kind: &'a str, attributes: &'a [TAttributes], children: &'a [TChildren]) -> Element<'a, TAttributes, TChildren> {
//         Element {
//             kind,
//             attributes,
//             children,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn should() {
//         let element1 = Element::new("element-a", &[], &[]);
//         let element2 = Element::new("element-a", &[], &[ &element1 ]);
//         let element3 = Element::new("element-a", &[], &[ &Element::new("element-b", &[], &[])]);

//         println!("{0:?}", element1);
//         println!("{0:?}", element2);
//         println!("{0:?}", element3);

//         assert!(false);
//     }
// }

// // let html1 = html(&[&Foo {}, &Bar {}, &Bar {}]);
// let body1 = body(&[&Bar {}, &Baz {}]);

// let html2 = html(
//     &[ &Baz {}
//     ],
//     &[ &body1
//     ]
// );

// // println!("{:?}", html1);
// println!("{:?}", body1);

// println!("{:?}", html2);

// pub trait IHtmlAttribute: std::fmt::Debug {}

// impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

// pub trait IHtmlChild: std::fmt::Debug {}

// impl<'a> IHtmlChild for &'a IHtmlChild {}

// #[derive(Clone, Debug)]
// pub struct Html<'a, TAttributes, TChildren>(Element<'a, TAttributes, TChildren>)
// where
//     TAttributes: IHtmlAttribute + Clone,
//     &'a IHtmlAttribute: From<TAttributes>,
//     TChildren: IHtmlChild + Clone,
//     &'a IHtmlChild: From<TChildren>;

// // pub struct Html<'a>(Element<&'a IHtmlAttribute>);

// pub trait IBodyAttribute: std::fmt::Debug {}

// impl<'a> IBodyAttribute for &'a IBodyAttribute {}

// pub trait IBodyChild: std::fmt::Debug {}

// impl<'a> IBodyChild for &'a IBodyChild {}

// #[derive(Clone, Debug)]
// pub struct Body<'a, TAttributes, TChildren>(Element<'a, TAttributes, TChildren>)
// where
//     TAttributes: IBodyAttribute + Clone,
//     &'a IBodyAttribute: From<TAttributes>,
//     TChildren: IBodyChild + Clone,
//     &'a IBodyChild: From<TChildren>;

// impl<'a, TAttributes, TChildren> IHtmlChild for Body<'a, TAttributes, TChildren>
// where
//     TAttributes: IBodyAttribute + Clone,
//     TChildren: IBodyChild + Clone
// {}

// // impl<'a, T> IElement for Html<'a>
// // where
// //     T: IHtmlAttribute + Clone,
// // {
// //     type TAttributes = T;
// // }

// #[derive(Clone, Debug)]
// pub struct Foo {}

// #[derive(Clone, Debug)]
// pub struct Bar {}

// #[derive(Clone, Debug)]
// pub struct Baz {}

// impl IHtmlAttribute for Foo {}

// impl IHtmlAttribute for Bar {}

// impl IBodyAttribute for Bar {}

// impl IBodyAttribute for Baz {}

// // pub fn html<'a, 'b: 'a, T: 'b>(attributes: &'b [&IHtmlAttribute]) -> Html<T>
// pub fn html<'a, TAttributes: 'a, TChildren: 'a>(
//     attributes: &'a [TAttributes],
//     children: &'a [TChildren]
// ) -> Html<'a, TAttributes, TChildren>
// where
//     TAttributes: IHtmlAttribute + Clone,
//     &'a IHtmlAttribute: From<TAttributes>,
//     TChildren: IHtmlChild + Clone,
//     &'a IHtmlChild: From<TChildren>,

//     // &'b T: IHtmlAttribute + Clone,
//     // IHtmlAttribute: From<T>,
// {
//     Html(
//         Element::<TAttributes, TChildren>{
//             kind: "html".to_owned(),
//             attributes: attributes.to_vec(),
//             children: children.to_vec(),
//             // attributes: attributes.to_vec().into_iter().map(|item| item.into()).collect(),
//         }
//     )
// }

// pub fn body<'a, TAttributes: 'a, TChildren: 'a>(
//     attributes: &'a [TAttributes],
//     children: &'a [TChildren]
// ) -> Body<'a, TAttributes, TChildren>
// where
//     TAttributes: IBodyAttribute + Clone,
//     &'a IBodyAttribute: From<TAttributes>,
//     TChildren: IBodyChild + Clone,
//     &'a IBodyChild: From<TChildren>,
// {
//     Body(
//         Element::<TAttributes, TChildren>{
//             kind: "body".to_owned(),
//             attributes: attributes.to_vec(),
//             children: children.to_vec(),
//         }
//     )
// }

