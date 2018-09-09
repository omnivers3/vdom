// #![no_std]

// pub trait IAttribute {}
pub trait IAttribute: std::fmt::Debug {}

impl<'a> IAttribute for &'a IAttribute {}

// pub trait IElement {}
pub trait IElement: std::fmt::Debug {}

impl<'a> IElement for &'a IElement {}

#[derive(Clone, Debug)]
pub enum ElementKinds<'a> {
    Custom (&'a str),
    Html,
    Body,
}

#[derive(Clone, Debug)]
pub struct Element<'a, TAttributes: 'a, TChildren: 'a>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{
    // pub kind: &'a str,
    pub kind: &'a str,
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
    // pub fn new(kind: &'a str, attributes: &'a [TAttributes], children: &'a [TChildren]) -> Element<'a, TAttributes, TChildren> {
    pub fn new(kind: &'a str, attributes: &'a [TAttributes], children: &'a [TChildren]) -> Element<'a, TAttributes, TChildren> {
        Element {
            kind,
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> From<Element<'a, TAttributes, TChildren>> for ElementKinds<'a>
where
    TAttributes: IAttribute + Clone,
    &'a IAttribute: From<TAttributes>,
    TChildren: IElement + Clone,
    &'a IElement: From<TChildren>,
{
    fn from(src: Element<'a, TAttributes, TChildren>) -> ElementKinds<'a> {
        match src.kind {
            "html" => ElementKinds::Html,
            _ => ElementKinds::Custom(src.kind),
        }
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

#[derive(Clone, Debug)]
pub struct HtmlElement<'a, TAttributes: 'a, TChildren: 'a>
where
{
    // pub kind: &'a str,
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
        Element::new("html", src.attributes, src.children)
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

#[derive(Clone, Debug)]
pub struct BodyElement<'a, TAttributes: 'a, TChildren: 'a>
where
{
    // pub kind: &'a str,
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
            // kind: "body",
            attributes,
            children,
        }
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