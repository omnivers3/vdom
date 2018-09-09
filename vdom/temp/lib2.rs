

// #![feature(associated_type_defaults)]
// #![feature(try_from)]

// #[macro_use]
// extern crate serde_derive;

// use std::marker::{ PhantomData };
// // use std::collections::hash_map::{ DefaultHasher, HashMap };
// use std::convert::{ TryFrom, TryInto };
// use std::fmt::{ Debug };
// // use std::hash::{Hash, Hasher};
// use std::hash::{Hash};

// /// Define the expected global trait set for events
// pub trait IEventData: Eq + Hash + PartialEq + Debug {}

// /// Auto-implement IEventData for types which share the required traits
// impl<TTarget> IEventData for TTarget
// where
//     TTarget: Eq + Hash + PartialEq + Debug,
// {}

// // #[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
// // pub struct Event<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     pub event_type: EventTypes,
// //     pub data: TEvents,
// // }

// // impl <TEvents> Event<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     pub fn map<TWrappingEvents, FnWrapper>(
// //         self,
// //         wrapper: FnWrapper,
// //     ) -> Event<TWrappingEvents>
// //     where
// //         TWrappingEvents: IEventData,
// //         FnWrapper: FnOnce(TEvents) -> TWrappingEvents + Clone,
// //     {
// //         Event {
// //             event_type: self.event_type,
// //             data: wrapper(self.data),
// //         }
// //     }
// // }

// // pub trait IEvent<TEvents>: Into<Event<TEvents>> where TEvents: IEventData {}

// pub trait IEvent<TEvents>: Into<EventTypes<TEvents>> where TEvents: IEventData {}

// #[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct OnClickEvent<TEvents>(TEvents) where TEvents: IEventData;

// #[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub enum EventTypes<TEvents>
// where
//     TEvents: IEventData,
// {
//     OnClick (OnClickEvent<TEvents>),
// }

// impl<TEvents> From<OnClickEvent<TEvents>> for EventTypes<TEvents>
// where
//     TEvents: IEventData,
// {
//     fn from(src: OnClickEvent<TEvents>) -> EventTypes<TEvents> {
//         EventTypes::OnClick (src)
//     }
// }


// pub struct Html<'a, TEvents>
// where
//     TEvents: IEventData
// {
//     events: PhantomData<TEvents>,
// }

// impl<'a, TEvents> Html<'a, TEvents>
// where
//     TEvents: IEventData,
// {
//     pub fn new() -> &'a Self 
//     // pub fn html<TChildren>(_attributes: &[HtmlAttributes<TEvents>], _children: &[TChildren]) -> HtmlElement
//     // where
//     //     TChildren: Into<HtmlElements>,
//     // {
//     //     HtmlElement {

//     //     }
//     // }

//     // pub fn body(_attributes: &[BodyAttributes<TEvents>], _children: &[BodyElements]) -> BodyElement {
//     //     BodyElement {

//     //     }
//     // }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn should_compose() {
//         // let e = element::<()>(HtmlElement{}.into(), &[], &[]);
//         // println!("{0:?}", e);

//         // let html_element = html::<(), _>(&[], &[body(&[], &[])]);
//         let html_element = H::html(&[], &[]);
//         println!("{0:?}", html_element);

//         // let html_element = html2(&[], &[]);
//         // let html_element = html::<()>(&[], &[html::<()>(&[], &[])]);
//         // println!("{0:?}", html_element);

//         assert!(false);
//     }
// }


// // pub trait IAttribute<TTarget> {
// //     type TAttributes;

// //     fn convert(self) -> Self::TAttributes;
// // }

// // // pub trait IAttribute<TEvents>: Into<AttributeTypes<TEvents>> where TEvents: IEventData {}

// // // impl<TEvents> IAttribute<TEvents> for AttributeTypes<TEvents> where TEvents: IEventData {}
// // impl<TEvents> IAttribute<AttributeTypes<TEvents>> for AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     type TAttributes = AttributeTypes<TEvents>;

// //     fn convert(self) -> AttributeTypes<TEvents> {
// //         self
// //     }
// // }

// // #[derive(Clone, Debug)]
// // pub struct CustomAttribute {
// //     pub key: String,
// //     pub value: String,
// // }

// // // impl<TEvents> IAttribute<TEvents> for CustomAttribute where TEvents: IEventData {}

// // #[derive(Clone, Debug)]
// // pub struct ClassAttribute {
// //     pub values: Vec<String>,
// // }

// // // impl<TEvents> IAttribute<TEvents> for ClassAttribute where TEvents: IEventData {}

// // #[derive(Clone, Debug)]
// // pub struct EventAttribute<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     event: EventTypes<TEvents>,
// // }

// // // impl<TEvents> IAttribute<TEvents> for EventAttribute<TEvents> where TEvents: IEventData {}

// // #[derive(Clone, Debug)]
// // pub enum AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     Custom (CustomAttribute),
// //     Class (ClassAttribute),
// //     Event (EventAttribute<TEvents>),
// // }

// // impl<TEvents> From<CustomAttribute> for AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     fn from(src: CustomAttribute) -> AttributeTypes<TEvents> {
// //         AttributeTypes::Custom (src)
// //     }
// // }

// // impl<TEvents> From<ClassAttribute> for AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     fn from(src: ClassAttribute) -> AttributeTypes<TEvents> {
// //         AttributeTypes::Class (src)
// //     }
// // }

// // impl<TEvents> From<EventAttribute<TEvents>> for AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     fn from(src: EventAttribute<TEvents>) -> AttributeTypes<TEvents> {
// //         AttributeTypes::Event (src)
// //     }
// // }

// // pub enum HtmlAttributes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     Custom (CustomAttribute),
// //     Event (EventAttribute<TEvents>),
// // }

// // impl<TEvents> From<HtmlAttributes<TEvents>> for AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     fn from(src: HtmlAttributes<TEvents>) -> AttributeTypes<TEvents> {
// //         match src {
// //             HtmlAttributes::Custom (a) => a.into(),
// //             HtmlAttributes::Event (a) => a.into(),
// //         }
// //     }
// // }

// // pub enum BodyAttributes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     Custom (CustomAttribute),
// //     Event (EventAttribute<TEvents>),
// // }

// // // pub trait IElement<TEvents>: Into<Element<TEvents>> where TEvents: IEventData {}

// // #[derive(Clone, Debug)]
// // pub struct CustomElement {
// //     pub kind: String,
// // }

// // #[derive(Clone, Debug)]
// // pub struct HtmlElement {}

// // #[derive(Clone, Debug)]
// // pub struct BodyElement {}

// // #[derive(Clone, Debug)]
// // pub struct DivElement {}

// // #[derive(Clone, Debug)]
// // pub enum ElementKinds {
// //     Custom (CustomElement),
// //     Html (HtmlElement),
// //     Body (BodyElement),
// //     Div (DivElement),
// // }

// // impl From<CustomElement> for ElementKinds {
// //     fn from(src: CustomElement) -> ElementKinds {
// //         ElementKinds::Custom (src)
// //     }
// // }

// // impl From<HtmlElement> for ElementKinds {
// //     fn from(src: HtmlElement) -> ElementKinds {
// //         ElementKinds::Html (src)
// //     }
// // }

// // impl From<BodyElement> for ElementKinds {
// //     fn from(src: BodyElement) -> ElementKinds {
// //         ElementKinds::Body (src)
// //     }
// // }

// // impl From<DivElement> for ElementKinds {
// //     fn from(src: DivElement) -> ElementKinds {
// //         ElementKinds::Div (src)
// //     }
// // }

// // #[derive(Clone, Debug)]
// // pub enum HtmlElements {
// //     Body (BodyElement),
// // }

// // impl From<HtmlElements> for ElementKinds {
// //     fn from(src: HtmlElements) -> ElementKinds {
// //         match src {
// //             HtmlElements::Body (e) => e.into(),
// //         }
// //     }
// // }

// // #[derive(Clone, Debug)]
// // pub enum BodyElements {
// //     Div (DivElement),
// // }

// // impl From<BodyElements> for ElementKinds {
// //     fn from(src: BodyElements) -> ElementKinds {
// //         match src {
// //             BodyElements::Div (e) => e.into(),
// //         }
// //     }
// // }

// // pub enum Errors {

// // }

// // // impl Error for Errors {

// // // }

// // impl TryFrom<BodyElements> for HtmlElements {
// //     type Error = ();

// //     fn try_from(src: BodyElements) -> Result<HtmlElements, ()> {
// //         match src {
// //             BodyElements::Div (_) => Err (())
// //         }
// //     }
// // }
// // pub fn class(values: &[&str]) -> ClassAttribute {
// //     let values = values.into_iter().map(|item| (*item).to_owned()).collect();
// //     ClassAttribute {
// //         values,
// //     }
// // }

// // #[derive(Clone, Debug)]
// // pub struct Node<TEvents, TAttributes>
// // where
// //     TEvents: IEventData,
// //     TAttributes: Into<AttributeTypes<TEvents>>,
// // {
// //     _events: PhantomData<TEvents>,
// //     kind: ElementKinds,
// //     attributes: Vec<TAttributes>,
// //     children: Vec<Node<TEvents, TAttributes>>,
// // }

// // impl<TEvents, TAttributes> Node<TEvents, TAttributes>
// // where
// //     TEvents: IEventData + Clone,
// //     TAttributes: Into<AttributeTypes<TEvents>> + Clone,
// // {
// //     pub fn new(kind: ElementKinds, attributes: &[TAttributes], children: &[Node<TEvents, TAttributes>]) -> Self {
// //         Node {
// //             _events: PhantomData,
// //             kind,
// //             attributes: attributes.to_vec(),
// //             children: children.to_vec(),
// //         }
// //     }
// // }

// // pub fn element<TEvents>(kind: ElementKinds, attributes: &[AttributeTypes<TEvents>], children: &[Node<TEvents, AttributeTypes<TEvents>>]) -> Node<TEvents, AttributeTypes<TEvents>>
// // where
// //     TEvents: IEventData + Clone,
// // {
// //     Node::new(kind, attributes, children)
// //     // Node {
// //     //     kind,
// //     //     attributes: attributes.to_vec(),
// //     //     children: children.to_vec(),
// //     // }
// // }

// // pub fn html<TEvents>(attributes: &[AttributeTypes<TEvents>], children: &[Node<TEvents, HtmlAttributes<TEvents>>]) -> Node<TEvents, AttributeTypes<TEvents>>
// // where
// //     TEvents: IEventData + Clone,
// // {
// //     // let children = children.into_iter().map(|item| (*item).into()).collect();
// //     Node::new(HtmlElement{}.into(), attributes, &[])
// //     // Node {
// //     //     kind: HtmlElement{}.into(),
// //     //     attributes: attributes.to_vec(),
// //     //     children: children.to_vec(),
// //     // }
// // }

// // pub trait IElement {
// //     type Model = ();
// //     type Events = ();
// // }

// // pub struct HtmlElement2 {}

// // impl IElement for HtmlElement2 {

// // }

// // pub fn html2(attributes: &[String], children: &[String]) -> HtmlElement2 {
// //     HtmlElement2 {}
// // }

// // pub struct BodyElement2 {}

// // impl IElement for BodyElement2 {

// // }

// // fn calculate_hash<T: Hash>(t: &T) -> u64 {
// //     let mut s = DefaultHasher::new();
// //     t.hash(&mut s);
// //     s.finish()
// // }

// // pub trait IDom<TEvents>: Debug
// // where
// //     TEvents: IEventData,
// // {
// // }
// //     type TAttributes: IAttribute<TEvents> + Clone + Debug;
// //     type TChildren: String;//IElement<TEvents> + Clone + Debug;

// //     fn element(&self, kind: &str, attributes: &[Self::TAttributes], children: &[Self::TChildren]);
// //     fn html(&self, attributes: &[Self::TAttributes], children: &[Self::TChildren]);
// // }

// // #[derive(Debug)]
// // pub struct HashDom<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     pub nodes: Option<TEvents>,
// // }

// // impl<TEvents> HashDom<TEvents>
// // where
// //     TEvents: IEventData + Clone,
// // {
// //     fn new() -> impl IDom<TEvents> {
// //         HashDom {
// //             nodes: None,
// //         }
// //     }
// // }

// // impl<TEvents> IDom<TEvents> for HashDom<TEvents>
// // where
// //     TEvents: IEventData + Clone,
// // {
// //     type TAttributes = AttributeTypes<TEvents>;
// //     type TChildren = String;//Node<AttributeTypes<TEvents>>;
    
// //     fn element(&self, kind: &str, attributes: &[Self::TAttributes], children: &[Self::TChildren]) {
// //         println!("html: {0:?}", attributes);
// //         let t: Vec<AttributeTypes<TEvents>> = attributes.into_iter().map(|i| (*i).clone().into()).collect();
// //     }

// //     fn html(&self, attributes: &[Self::TAttributes], children: &[Self::TChildren]) -> () {
// //         self.element("html", attributes, children);
// //     }
// // }

// // pub trait IAttribute<TTarget> {
// //     type TAttributes;

// //     fn convert(self) -> Self::TAttributes;
// // }

// // // pub trait IAttribute<TEvents>: Into<AttributeTypes<TEvents>> where TEvents: IEventData {}

// // // impl<TEvents> IAttribute<TEvents> for AttributeTypes<TEvents> where TEvents: IEventData {}
// // impl<TEvents> IAttribute<AttributeTypes<TEvents>> for AttributeTypes<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     type TAttributes = AttributeTypes<TEvents>;

// //     fn convert(self) -> AttributeTypes<TEvents> {
// //         self
// //     }
// // }

// // pub trait IAttribute<TEvents>: Debug//: Into<AttributeTypes<TEvents>>
// // where
// //     TEvents: IEventData,
// // {
// //     fn to_attribute(&self) -> AttributeTypes<TEvents>;
// // }

// // impl<T, TEvents> IAttribute<TEvents> for T
// // where
// //     TEvents: IEventData,
// //     T: Into<AttributeTypes<TEvents>> + Clone + Debug
// // {
// //     fn to_attribute(&self) -> AttributeTypes<TEvents> {
// //         (*self).to_owned().into()
// //     }
// // }

// // pub trait IElement: Debug {
// //     fn to_element(&self) -> ElementKinds;
// // }

// // impl<T> IElement for T
// // where T: Into<ElementKinds> + Clone + Debug
// // {
// //     fn to_element(&self) -> ElementKinds {
// //         (*self).to_owned().into()
// //     }
// // }

// // pub trait INode<TEvents>: Debug
// // where
// //     TEvents: IEventData,
// // {
// //     fn to_node(&self) -> Node<TEvents>;
// // }

// // #[derive(Clone, Debug)]
// // pub struct Node<'a, TEvents: 'a>
// // where
// //     TEvents: IEventData,
// // {
// //     pub kind: &'a IElement,
// //     pub attributes: Vec<&'a IAttribute<TEvents>>,
// //     pub children: Vec<&'a INode<TEvents>>,
// // }

// // impl<'a, TEvents> INode<TEvents> for Node<'a, TEvents>
// // where
// //     TEvents: IEventData,
// //     Self: Clone,
// // {
// //     fn to_node(&self) -> Node<'_, TEvents> {
// //         (*self).to_owned()
// //     }
// // }

// // pub fn element<'a, TEvents: 'a>(
// //     kind: &'a IElement,
// //     attributes: &[&'a IAttribute<TEvents>],
// //     children: &[&'a INode<TEvents>],
// // ) -> Node<'a, TEvents>
// // where
// //     TEvents: IEventData + Debug,
// //     // TAttributes: IAttribute<TEvents> + Debug,
// // {
// //     // let kind = kind.to_owned();
// //     // let attributes = attributes.into_iter().map(|item| (*item).to_attribute()).collect();
// //     // let children = children.into_iter().map(|item| (*item).to_element()).collect();
// //     // (kind, attributes, children)
// //     Node::<TEvents> {
// //         kind,
// //         attributes: attributes.to_vec(),
// //         children: children.to_vec(),
// //     }
// // }
// // // pub fn element<TEvents, TAttributes, TChildren>(kind: &str, attributes: &[TAttributes], children: &[TChildren])
// // // where
// // //     TEvents: IEventData,
// // //     TAttributes: IAttribute<TEvents>,
// // //     TChildren: FnOnce(IDom<TEvents>) -> Node<>
// // // {

// // // }

// // #[derive(Clone, Debug)]
// // pub struct DomArena<'a, TEvents: 'a>
// // where
// //     TEvents: IEventData,
// // {
// //     attributes: HashMap<u64, Rc<&'a IAttribute<TEvents>>,
// //     nodes: HashMap<u64, &'a INode<TEvents>>,
// // }

// // impl<'a, TEvents> DomArena<'a, TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     pub fn new() -> Self {
// //         DomArena {
// //             attributes: HashMap::new(),
// //             nodes: HashMap::new(),
// //         }
// //     }
// // }

// // pub trait IDom<'a, TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     fn element(
// //         &mut self,
// //         kind: &'a IElement,
// //         attributes: &[&'a IAttribute<TEvents>],
// //         children: &[&'a INode<TEvents>]
// //     ) -> &'a INode<TEvents>;
// // }

// // impl<'a, TEvents> IDom<'a, TEvents> for DomArena<'a, TEvents>
// // where
// //     TEvents: IEventData + Clone,
// // {
// //     fn element(
// //         &mut self,
// //         kind: &'a IElement,
// //         attributes: &[&'a IAttribute<TEvents>],
// //         children: &[&'a INode<TEvents>]
// //     ) -> &'a INode<TEvents> {
// //         let node = Node {
// //             kind: &HtmlElement {},
// //             attributes: vec![],
// //             children: vec![],
// //         };
// //         self.nodes.insert(0, &node);
// //         (*self.nodes.get(&0).unwrap())
// //     }
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn should_compose() {
// //         let mut h = DomArena::<()>::new();
// //         println!("{0:?}", h);

// //         let he = h.element(&HtmlElement{}, &[], &[]);

// //         let e = element::<()>(&HtmlElement {}, &[], &[]);
// //         let class_attribute = &class(&["green_button", "black_button"]);
// //         let custom_attribute = &CustomAttribute {
// //             key: "asdf".to_owned(),
// //             value: "fdsa".to_owned(),
// //         };
// //         let e1 = element::<()>(&HtmlElement {}, &[
// //             // class(&["green_button", "black_button"]),
// //             class_attribute,
// //             custom_attribute,
// //         ], &[]);
        
// //         println!("{0:?}", e);
// //         println!("{0:?}", e1);

// //         // let hash_dom = HashDom::<()>::new();
// //         // let h = Dom::<()>::new();

// //         // let html1 = html(&[], &[]);
// //         // let html2 = html(&[], &[html1.clone()]);

// //         // element("html", &[], &[]);


// //         // let e: Element<()> = h.element("asdf",
// //         //     &[ ClassAttribute { values: vec!["blue_button".to_owned()] }.into()
// //         //     , class(&["green_button", "black_button"])
// //         //     ],
// //         //     &[ Html::element("blah", &[], &[])

// //         //     ]);

// //         // println!("{0:?}", html1);

// //         assert!(false);
// //     }

// //     // #[test]
// //     // fn should_element() {
// //     //     let element1 = Element::<()>::new("element-a", &[], &[]);
// //     //     let element2 = Element::new("element-a", &[], &[element1.clone()]);
// //     //     let element3 = Element::<()>::new("element-a", &[], &[Element::new("element-b", &[], &[])]);

// //     //     println!("{0:?}", element1);
// //     //     println!("{0:?}", element2);
// //     //     println!("{0:?}", element3);

// //     //     assert!(true);
// //     // }
// // }

// // pub trait INode {
// //     type TDatum = ();

// //     fn children() -> Vec<INode<TDatum=Self::TDatum>>;
// // }

// // #[derive(Clone, Debug)]
// // pub struct Node<TEvents, TAttributes, TChildren> {
// //     pub element: ElementKinds,
// //     pub attributes: Vec<AttributeTypes<TEvents>>,
// //     pub children: Vec<Node<TEvents, TAttributes, TChildren>>,
// // }


// // pub type Element<TEvents> = Node<TEvents, AttributeTypes<TEvents>, A>;

// // #[derive(Clone, Debug)]
// // pub struct Node<TDatum> {
// //     pub datum: TDatum,
// //     pub children: Vec<Node<TDatum>>,
// // }

// // #[derive(Clone, Debug)]
// // pub struct ElementDatum<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     pub element: ElementKinds,
// //     pub attributes: Vec<AttributeTypes<TEvents>>,
// // }

// // pub type Element<TEvents> = Node<ElementDatum<TEvents>>;

// // impl<TEvents> From<Node<ElementDatum<TEvents>>> for ElementKinds
// // where
// //     TEvents: IEventData,
// // {
// //     fn from(src: Node<ElementDatum<TEvents>>) -> ElementKinds {
// //         src.into()
// //     }
// // }

// // impl<TEvents> IElement<TEvents> for Element<TEvents> where TEvents: IEventData {}

// // impl<TEvents> Element<TEvents>
// // where
// //     TEvents: IEventData,
// // {
// //     pub fn new(datum: ElementDatum<TEvents>, children: Vec<Node<ElementDatum<TEvents>>>) -> Self {
// //         Node::<ElementDatum<TEvents>> {
// //             datum,
// //             children,
// //         }
// //     }
// // }



// // pub trait IElement<TEvents>: Into<ElementKinds> where TEvents: IEventData {}


// // pub fn element<TEvents, TAttributes, TChildren>(kind: &str, attributes: &[TAttributes], children: &[TChildren]) -> Element<TEvents>
// // where
// //     TEvents: IEventData + Clone,
// //     TAttributes: IAttribute<TEvents> + Clone,
// //     TChildren: IElement<TEvents> + Clone,
// // {
// //     let element = CustomElement { kind: kind.to_owned() }.into();
// //     let attributes = attributes.into_iter().map(|item| (*item).to_owned().into()).collect();
// //     let children = children.into_iter().map(|item| {
// //         let temp: Element<TEvents> = (*item).to_owned().into();
// //         temp
// //     }).collect();
// //     // let children = children.to_vec();

// //     Element::<TEvents>::new(
// //         ElementDatum::<TEvents> {
// //             element,
// //             attributes,
// //         },
// //         children,
// //     )
// // }

// // #[derive(Clone, Debug)]
// // pub struct Empty {}

// // pub struct Html<TEvents, TAttributes=CustomAttribute, TChildren=Empty>
// // where
// //     TEvents: IEventData + Clone,
// //     TAttributes: IAttribute<TEvents> + Clone,
// //     TChildren: IElement<TEvents> + Clone,
// // {
// //     events: PhantomData<TEvents>,
// //     attributes: PhantomData<TAttributes>,
// //     children: PhantomData<TChildren>,
// // }

// // impl<TEvents, TAttributes, TChildren> Html<TEvents, TAttributes, TChildren>
// // where
// //     TEvents: IEventData + Clone,
// //     TAttributes: IAttribute<TEvents> + Clone,
// //     TChildren: IElement<TEvents> + Clone,
// // {
// //     pub fn element(kind: &str, attributes: &[TAttributes], children: &[TChildren]) -> Element<TEvents>
// //     {
// //         let element = CustomElement { kind: kind.to_owned() }.into();
// //         let attributes = attributes.into_iter().map(|item| (*item).to_owned().into()).collect();
// //         let children = children.into_iter().map(|item| {
// //             let temp: Element<TEvents> = (*item).to_owned().into();
// //             temp
// //         }).collect();
// //         // let children = children.to_vec();

// //         Element::<TEvents>::new(
// //             ElementDatum::<TEvents> {
// //                 element,
// //                 attributes,
// //             },
// //             children,
// //         )
// //     }
// // }

// // pub fn element<TEvents>(kind: &str, attributes: &[AttributeTypes<TEvents>], children: &[Element<TEvents>]) -> Element<TEvents>
// // where
// //     TEvents: IEventData + Clone,
// // {
// //     let element = CustomElement { kind: kind.to_owned() }.into();
// //     let attributes = attributes.to_vec();
// //     let children = children.to_vec();
// //     Element::<TEvents>::new(
// //         ElementDatum::<TEvents> {
// //             element,
// //             attributes,
// //         },
// //         children,
// //     )
// // }

// // pub trait IHtmlAttribute: std::fmt::Debug {}

// // impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

// // pub trait IHtmlChild: std::fmt::Debug {}

// // impl<'a> IHtmlChild for &'a IHtmlChild {}

// // #[derive(Clone, Debug)]
// // pub enum HtmlAttributes {
// // }

// // #[derive(Clone, Debug)]
// // pub enum HtmlChildren {
// //     Body (BodyChildren),
// // }

// // #[derive(Clone, Debug)]
// // pub enum BodyAttributes {

// // }

// // #[derive(Clone, Debug)]
// // pub enum BodyChildren {

// // }

// // impl From<BodyChildren> for HtmlChildren {
// //     fn from(source: BodyChildren) -> HtmlChildren {
// //         HtmlChildren::Body (source)
// //     }
// // }

// // #[derive(Clone, Debug)]
// // pub struct HtmlElement<TAttributes>(Element<TAttributes>);

// // pub fn html<TAttributes, TChildren>(
// //     attributes: &[TAttributes],
// //     children: &[TChildren]
// // ) -> HtmlElement<TAttributes>
// // where
// //     TAttributes: Into<HtmlAttributes> + Clone + std::fmt::Debug,
// //     TChildren: Clone + std::fmt::Debug,
// // {
// //     let children = children
// //         .into_iter()
// //         .map(|item| {
// //             let out: Element<TAttributes> = (*item).into();
// //             out
// //         })
// //         .collect()
// //         .to_vec();
// //     HtmlElement::<TAttributes>(
// //         Element::new("html", attributes, children)
// //     )
// // }

// // pub trait IElement {
// //     type TAttributes;
// // }

// // pub trait IAttribute: std::fmt::Debug {}

// // impl<'a> IAttribute for &'a IAttribute {}

// // pub trait IElement: std::fmt::Debug {}

// // impl<'a> IElement for &'a IElement {}

// // #[derive(Clone, Debug)]
// // pub struct Element<'a, TAttributes: 'a, TChildren: 'a>
// // where
// //     TAttributes: IAttribute + Clone,
// //     &'a IAttribute: From<TAttributes>,
// //     TChildren: IElement + Clone,
// //     &'a IElement: From<TChildren>,
// // {
// //     pub kind: &'a str,
// //     pub attributes: &'a [TAttributes],
// //     pub children: &'a [TChildren],
// // }

// // impl<'a, TAttributes: 'a, TChildren: 'a> IElement for Element<'a, TAttributes, TChildren>
// // where
// //     TAttributes: IAttribute + Clone,
// //     &'a IAttribute: From<TAttributes>,
// //     TChildren: IElement + Clone,
// //     &'a IElement: From<TChildren>,
// // {}

// // impl<'a, TAttributes: 'a, TChildren: 'a> Element<'a, TAttributes, TChildren>
// // where
// //     TAttributes: IAttribute + Clone,
// //     &'a IAttribute: From<TAttributes>,
// //     TChildren: IElement + Clone,
// //     &'a IElement: From<TChildren>,
// // {
// //     fn new(kind: &'a str, attributes: &'a [TAttributes], children: &'a [TChildren]) -> Element<'a, TAttributes, TChildren> {
// //         Element {
// //             kind,
// //             attributes,
// //             children,
// //         }
// //     }
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn should() {
// //         let element1 = Element::new("element-a", &[], &[]);
// //         let element2 = Element::new("element-a", &[], &[ &element1 ]);
// //         let element3 = Element::new("element-a", &[], &[ &Element::new("element-b", &[], &[])]);

// //         println!("{0:?}", element1);
// //         println!("{0:?}", element2);
// //         println!("{0:?}", element3);

// //         assert!(false);
// //     }
// // }

// // // let html1 = html(&[&Foo {}, &Bar {}, &Bar {}]);
// // let body1 = body(&[&Bar {}, &Baz {}]);

// // let html2 = html(
// //     &[ &Baz {}
// //     ],
// //     &[ &body1
// //     ]
// // );

// // // println!("{:?}", html1);
// // println!("{:?}", body1);

// // println!("{:?}", html2);

// // pub trait IHtmlAttribute: std::fmt::Debug {}

// // impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

// // pub trait IHtmlChild: std::fmt::Debug {}

// // impl<'a> IHtmlChild for &'a IHtmlChild {}

// // #[derive(Clone, Debug)]
// // pub struct Html<'a, TAttributes, TChildren>(Element<'a, TAttributes, TChildren>)
// // where
// //     TAttributes: IHtmlAttribute + Clone,
// //     &'a IHtmlAttribute: From<TAttributes>,
// //     TChildren: IHtmlChild + Clone,
// //     &'a IHtmlChild: From<TChildren>;

// // // pub struct Html<'a>(Element<&'a IHtmlAttribute>);

// // pub trait IBodyAttribute: std::fmt::Debug {}

// // impl<'a> IBodyAttribute for &'a IBodyAttribute {}

// // pub trait IBodyChild: std::fmt::Debug {}

// // impl<'a> IBodyChild for &'a IBodyChild {}

// // #[derive(Clone, Debug)]
// // pub struct Body<'a, TAttributes, TChildren>(Element<'a, TAttributes, TChildren>)
// // where
// //     TAttributes: IBodyAttribute + Clone,
// //     &'a IBodyAttribute: From<TAttributes>,
// //     TChildren: IBodyChild + Clone,
// //     &'a IBodyChild: From<TChildren>;

// // impl<'a, TAttributes, TChildren> IHtmlChild for Body<'a, TAttributes, TChildren>
// // where
// //     TAttributes: IBodyAttribute + Clone,
// //     TChildren: IBodyChild + Clone
// // {}

// // // impl<'a, T> IElement for Html<'a>
// // // where
// // //     T: IHtmlAttribute + Clone,
// // // {
// // //     type TAttributes = T;
// // // }

// // #[derive(Clone, Debug)]
// // pub struct Foo {}

// // #[derive(Clone, Debug)]
// // pub struct Bar {}

// // #[derive(Clone, Debug)]
// // pub struct Baz {}

// // impl IHtmlAttribute for Foo {}

// // impl IHtmlAttribute for Bar {}

// // impl IBodyAttribute for Bar {}

// // impl IBodyAttribute for Baz {}

// // // pub fn html<'a, 'b: 'a, T: 'b>(attributes: &'b [&IHtmlAttribute]) -> Html<T>
// // pub fn html<'a, TAttributes: 'a, TChildren: 'a>(
// //     attributes: &'a [TAttributes],
// //     children: &'a [TChildren]
// // ) -> Html<'a, TAttributes, TChildren>
// // where
// //     TAttributes: IHtmlAttribute + Clone,
// //     &'a IHtmlAttribute: From<TAttributes>,
// //     TChildren: IHtmlChild + Clone,
// //     &'a IHtmlChild: From<TChildren>,

// //     // &'b T: IHtmlAttribute + Clone,
// //     // IHtmlAttribute: From<T>,
// // {
// //     Html(
// //         Element::<TAttributes, TChildren>{
// //             kind: "html".to_owned(),
// //             attributes: attributes.to_vec(),
// //             children: children.to_vec(),
// //             // attributes: attributes.to_vec().into_iter().map(|item| item.into()).collect(),
// //         }
// //     )
// // }

// // pub fn body<'a, TAttributes: 'a, TChildren: 'a>(
// //     attributes: &'a [TAttributes],
// //     children: &'a [TChildren]
// // ) -> Body<'a, TAttributes, TChildren>
// // where
// //     TAttributes: IBodyAttribute + Clone,
// //     &'a IBodyAttribute: From<TAttributes>,
// //     TChildren: IBodyChild + Clone,
// //     &'a IBodyChild: From<TChildren>,
// // {
// //     Body(
// //         Element::<TAttributes, TChildren>{
// //             kind: "body".to_owned(),
// //             attributes: attributes.to_vec(),
// //             children: children.to_vec(),
// //         }
// //     )
// // }

