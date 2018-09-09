// #![no_std]

// pub trait IAttribute {}

// pub trait IElement {}

pub trait IAttribute: std::fmt::Debug {}

pub trait IElement: std::fmt::Debug {}

impl<'a> IAttribute for &'a IAttribute {}

impl<'a> IElement for &'a IElement {}

pub static HtmlKind: &'static str = "html";
pub static BodyKind: &'static str = "body";

#[derive(Clone, Debug)]
pub enum ElementKinds {
    Custom (&'static str),
    Html,
    Body,
}

impl From<ElementKinds> for &'static str {
    fn from(src: ElementKinds) -> &'static str {
        match src {
            ElementKinds::Custom (kind) => kind,
            ElementKinds::Html => HtmlKind,
            ElementKinds::Body => BodyKind,
        }
    }
}

impl From<&'static str> for ElementKinds {
    fn from(src: &'static str) -> ElementKinds {
        match src {
            "html" => ElementKinds::Html,
            "body" => ElementKinds::Body,
            _ => ElementKinds::Custom(src),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Element<'a, TAttributes: 'a, TChildren: 'a>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{
    pub kind: &'static str,
    pub attributes: &'a [TAttributes],
    pub children: &'a [TChildren],
}

impl<'a, TAttributes: 'a, TChildren: 'a> Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{
    pub fn new(kind: &'static str, attributes: &'a [TAttributes], children: &'a [TChildren]) -> Element<'a, TAttributes, TChildren> {
        Element {
            kind,
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> From<Element<'a, TAttributes, TChildren>> for ElementKinds
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{
    fn from(src: Element<'a, TAttributes, TChildren>) -> ElementKinds {
        src.kind.into()
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> From<Element<'a, TAttributes, TChildren>> for &'static str
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{
    fn from(src: Element<'a, TAttributes, TChildren>) -> &'static str {
        let t: ElementKinds = src.kind.into();
        t.into()
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IElement for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{}

impl<'a, TAttributes: 'a, TChildren: 'a> IHtmlElement for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{}

impl<'a, TAttributes: 'a, TChildren: 'a> IBodyElement for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{}

pub trait IHtmlAttribute: IAttribute {}

impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

impl<'a> IAttribute for &'a IHtmlAttribute {}

pub trait IHtmlElement: IElement {}

impl<'a> IHtmlElement for &'a IHtmlElement {}

impl<'a> IElement for &'a IHtmlElement {}

impl<'a> IHtmlElement for &'a IElement {}

// impl<'a> From<&'a IHtmlElement> for ElementKinds {
//     fn from(src: &'a IHtmlElement) -> ElementKinds {
//         // ElementKinds::Html
//     }
// }

// impl<'a> From<&'a IHtmlElement> for &'static str {
//     fn from(src: &'a IHtmlElement) -> &'static str {
//         ElementKinds::Html.into()
//     }
// }

#[derive(Clone, Debug)]
pub struct HtmlElement<'a, TAttributes: 'a, TChildren: 'a> {
    pub attributes: &'a [TAttributes],
    pub children: &'a [TChildren],
}

impl<'a, TAttributes: 'a, TChildren: 'a> HtmlElement<'a, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute + IAttribute + Clone,
    &'a IHtmlAttribute: From<TAttributes>,
    TChildren: IHtmlElement + IElement + Clone,
    &'a IHtmlElement: From<TChildren>,
{
    pub fn new(attributes: &'a [TAttributes], children: &'a [TChildren]) -> Self {
        HtmlElement {
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> From<&'a HtmlElement<'a, TAttributes, TChildren>> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute + IAttribute + Clone,
    &'a IHtmlAttribute: From<TAttributes>,
    &'a IAttribute: From<TAttributes>,
    TChildren: IHtmlElement + IElement + Clone,
    &'a IHtmlElement: From<TChildren>,
    &'a IElement: From<TChildren>,
{
    fn from(src: &'a HtmlElement<'a, TAttributes, TChildren>) -> Element<'a, TAttributes, TChildren> {
        Element::new(HtmlKind, src.attributes, src.children)
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IElement for HtmlElement<'a, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute + IAttribute + Clone,
    &'a IHtmlAttribute: From<TAttributes>,
    TChildren: IHtmlElement + IElement + Clone,
    &'a IHtmlElement: From<TChildren>,
{}

pub trait IBodyAttribute: IAttribute {}

impl<'a> IBodyAttribute for &'a IBodyAttribute {}

impl<'a> IAttribute for &'a IBodyAttribute {}

pub trait IBodyElement: IElement {}

impl<'a> IBodyElement for &'a IBodyElement {}

impl<'a> IElement for &'a IBodyElement {}

impl<'a> IBodyElement for &'a IElement {}

// impl<'a> From<&'a IBodyElement> for ElementKinds {
//     fn from(src: &'a IBodyElement) -> ElementKinds {
//         // ElementKinds::Body
//     }
// }

// impl<'a> From<&'a IBodyElement> for &'static str {
//     fn from(src: &'a IBodyElement) -> &'static str {
//         ElementKinds::Body.into()
//     }
// }

#[derive(Clone, Debug)]
pub struct BodyElement<'a, TAttributes: 'a, TChildren: 'a>
where
{
    pub attributes: &'a [TAttributes],
    pub children: &'a [TChildren],
}

impl<'a, TAttributes: 'a, TChildren: 'a> BodyElement<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute + IAttribute + Clone,
    &'a IBodyAttribute: From<TAttributes>,
    TChildren: IBodyElement + IElement + Clone,
    &'a IBodyElement: From<TChildren>,
{
    pub fn new(attributes: &'a [TAttributes], children: &'a [TChildren]) -> Self {
        BodyElement {
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> From<&'a BodyElement<'a, TAttributes, TChildren>> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute + IAttribute + Clone,
    &'a IBodyAttribute: From<TAttributes>,
    &'a IAttribute: From<TAttributes>,
    TChildren: IBodyElement + IElement + Clone,
    &'a IBodyElement: From<TChildren>,
    &'a IElement: From<TChildren>,
{
    fn from(src: &'a BodyElement<'a, TAttributes, TChildren>) -> Element<'a, TAttributes, TChildren> {
        Element::new(BodyKind, src.attributes, src.children)
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IElement for BodyElement<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute + IAttribute + Clone,
    &'a IBodyAttribute: From<TAttributes>,
    TChildren: IBodyElement + IElement + Clone,
    &'a IBodyElement: From<TChildren>,
{}

impl<'a, TAttributes: 'a, TChildren: 'a> IHtmlElement for BodyElement<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute + Clone,
    &'a IBodyAttribute: From<TAttributes>,
    TChildren: IBodyElement + Clone,
    &'a IBodyElement: From<TChildren>,
{}

#[derive(Clone, Debug)]
pub struct ClassAttribute<'a> {
    pub value: &'a str,
}

impl<'a> IAttribute for ClassAttribute<'a> {}