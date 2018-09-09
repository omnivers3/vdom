#![no_std]

// pub trait IBase: std::fmt::Debug + PartialEq {}

// impl<T> IBase for T
// where
//     T: std::fmt::Debug + PartialEq,
// {}

pub trait IAttribute<'a,>{//: std::fmt::Debug {
    fn key(&self) -> &'a str;
}



// impl IBase for IAttribute {}

impl<'a> IAttribute<'a> for &'a IAttribute<'a> {
    fn key(&self) -> &'a str {
        (*self).key()
    }
}

// pub trait IElement {}
pub trait IElement<'a>{//: std::fmt::Debug {
    fn kind(&self) -> &'a str;
}

// impl IBase for IElement {}

impl<'a> IElement<'a> for &'a IElement<'a> {
    fn kind(&self) -> &'a str {
        (*self).kind()
    }
}

#[derive(Clone, Debug)]//, PartialEq, Eq)]
pub struct Element<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b>
where
    TAttributes: IAttribute<'b> + Clone,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IElement<'b> + Clone,
    &'b IElement<'b>: From<TChildren>,
{
    pub kind: &'a str,
    pub attributes: &'b [TAttributes],
    pub children: &'b [TChildren],
}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> Element<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IAttribute<'b> + Clone,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IElement<'b> + Clone,
    &'b IElement<'b>: From<TChildren>,
{
    pub fn new(kind: &'a str, attributes: &'b [TAttributes], children: &'b [TChildren]) -> Element<'a, 'b, TAttributes, TChildren> {
        Element {
            kind,
            attributes,
            children,
        }
    }
}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> IElement<'a> for Element<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IAttribute<'b> + Clone,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IElement<'b> + Clone,
    &'b IElement<'b>: From<TChildren>,
{
    fn kind(&self) -> &'a str {
        self.kind
        // &self.kind.to_owned()
    }
}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> IHtmlElement<'a> for Element<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IAttribute<'b> + Clone,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IElement<'b> + Clone,
    &'b IElement<'b>: From<TChildren>,
{}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> IBodyElement<'a> for Element<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IAttribute<'b> + Clone,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IElement<'b> + Clone,
    &'b IElement<'b>: From<TChildren>,
{}

pub trait IHtmlAttribute<'a>: IAttribute<'a> {}
// pub trait IHtmlAttribute: IAttribute<'a> + std::fmt::Debug {}

impl<'a> IHtmlAttribute<'a> for &'a IHtmlAttribute<'a> {}

impl<'a> IAttribute<'a> for &'a IHtmlAttribute<'a> {
    fn key(&self) -> &'a str {
        (*self).key()
    }
}

pub trait IHtmlElement<'a>: IElement<'a> {}//+ std::fmt::Debug {}

impl<'a> IHtmlElement<'a> for &'a IHtmlElement<'a> {}

impl<'a> IElement<'a> for &'a IHtmlElement<'a> {
    fn kind(&self) -> &'a str {
        (*self).kind()
    }
}

impl<'a> IHtmlElement<'a> for &'a IElement<'a> {}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> From<&'a IHtmlElement<'a>> for Element<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IAttribute<'b> + Clone,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IElement<'b> + Clone,
    &'b IElement<'b>: From<TChildren>,
{
    fn from(src: &'a IHtmlElement) -> Element<'a, 'b, TAttributes, TChildren> {
        Element {
            kind: src.kind(),
            attributes: &[],
            children: &[],
        }
    }
}

#[derive(Clone, Debug)]
pub struct HtmlElement<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b>
where
{
    pub kind: &'a str,
    pub attributes: &'b [TAttributes],
    pub children: &'b [TChildren],
}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> HtmlElement<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute<'b> + IAttribute<'b> + Clone,
    &'b IHtmlAttribute<'b>: From<TAttributes>,
    TChildren: IHtmlElement<'b> + IElement<'b> + Clone,
    &'b IHtmlElement<'b>: From<TChildren>,
{
    pub fn new(attributes: &'b [TAttributes], children: &'b [TChildren]) -> Self {
        HtmlElement {
            kind: "html",
            attributes,
            children,
        }
    }
}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> From<&'a HtmlElement<'a, 'b, TAttributes, TChildren>> for Element<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute<'b> + IAttribute<'b> + Clone,
    &'b IHtmlAttribute<'b>: From<TAttributes>,
    &'b IAttribute<'b>: From<TAttributes>,
    TChildren: IHtmlElement<'b> + IElement<'b> + Clone,
    &'b IHtmlElement<'b>: From<TChildren>,
    &'b IElement<'b>: From<TChildren>,
{
    fn from(src: &'a HtmlElement<'a, 'b, TAttributes, TChildren>) -> Element<'a, 'b, TAttributes, TChildren> {
        Element::new(src.kind, src.attributes, src.children)
    }
}

impl<'a, 'b: 'a, TAttributes: 'b, TChildren: 'b> IElement<'a> for HtmlElement<'a, 'b, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute<'b> + IAttribute<'b> + Clone,
    &'b IHtmlAttribute<'b>: From<TAttributes>,
    TChildren: IHtmlElement<'b> + IElement<'b> + Clone,
    &'b IHtmlElement<'b>: From<TChildren>,
{
    fn kind(&self) -> &'a str {
        self.kind
        // &self.kind.to_owned()
    }
}

pub trait IBodyAttribute<'a>: IAttribute<'a> {}//+ std::fmt::Debug {}

impl<'a> IBodyAttribute<'a> for &'a IBodyAttribute<'a> {}

impl<'a> IAttribute<'a> for &'a IBodyAttribute<'a> {
    fn key(&self) -> &'a str {
        (*self).key()//.to_owned()
    }
}

pub trait IBodyElement<'a>: IElement<'a> {}//+ std::fmt::Debug {}

impl<'a> IBodyElement<'a> for &'a IBodyElement<'a> {}

impl<'a> IElement<'a> for &'a IBodyElement<'a> {
    fn kind(&self) -> &'a str {
        (*self).kind()
    }
}

impl<'a> IBodyElement<'a> for &'a IElement<'a> {}

#[derive(Clone, Debug)]
pub struct BodyElement<'a, TAttributes: 'a, TChildren: 'a>
where
{
    pub kind: &'a str,
    pub attributes: &'a [TAttributes],
    pub children: &'a [TChildren],
}

impl<'a, TAttributes: 'a, TChildren: 'a> BodyElement<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute<'a> + IAttribute<'a> + Clone,
    &'a IBodyAttribute<'a>: From<TAttributes>,
    TChildren: IBodyElement<'a> + IElement<'a> + Clone,
    &'a IBodyElement<'a>: From<TChildren>,
{
    pub fn new(attributes: &'a [TAttributes], children: &'a [TChildren]) -> Self {
        BodyElement {
            kind: "body",
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IElement<'a> for BodyElement<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute<'a> + IAttribute<'a> + Clone,
    &'a IBodyAttribute<'a>: From<TAttributes>,
    TChildren: IBodyElement<'a> + IElement<'a> + Clone,
    &'a IBodyElement<'a>: From<TChildren>,
{
    fn kind(&self) -> &'a str {
        self.kind
        // &self.kind.to_owned()
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IHtmlElement<'a> for BodyElement<'a, TAttributes, TChildren>
where
    TAttributes: IBodyAttribute<'a> + Clone,
    &'a IBodyAttribute<'a>: From<TAttributes>,
    TChildren: IBodyElement<'a> + Clone,
    &'a IBodyElement<'a>: From<TChildren>,
{}

#[derive(Clone, Debug)]
pub struct ClassAttribute<'a> {
    pub value: &'a str,
}

impl<'a> IAttribute<'a> for ClassAttribute<'a> {
    fn key(&self) -> &'a str {
        "class"
        // &"class".to_owned()
    }
}