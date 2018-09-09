#![no_std]

// pub trait IBase: std::fmt::Debug + PartialEq {}

// impl<T> IBase for T
// where
//     T: std::fmt::Debug + PartialEq,
// {}

pub trait IAttribute<'a, 'b: 'a>{//: std::fmt::Debug {
    fn key(&self) -> &'b str;
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
pub struct Element<'a, TAttributes: 'a, TChildren: 'a>
where
    TAttributes: IAttribute<'a> + Clone,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IElement<'a> + Clone,
    &'a IElement<'a>: From<TChildren>,
{
    pub kind: &'a str,
    pub attributes: &'a [TAttributes],
    pub children: &'a [TChildren],
}

impl<'a, TAttributes: 'a, TChildren: 'a> Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute<'a> + Clone,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IElement<'a> + Clone,
    &'a IElement<'a>: From<TChildren>,
{
    pub fn new(kind: &'a str, attributes: &'a [TAttributes], children: &'a [TChildren]) -> Element<'a, TAttributes, TChildren> {
        Element {
            kind,
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IElement<'a> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute<'a> + Clone,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IElement<'a> + Clone,
    &'a IElement<'a>: From<TChildren>,
{
    fn kind(&self) -> &'a str {
        self.kind
        // &self.kind.to_owned()
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IHtmlElement<'a> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute<'a> + Clone,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IElement<'a> + Clone,
    &'a IElement<'a>: From<TChildren>,
{}

impl<'a, TAttributes: 'a, TChildren: 'a> IBodyElement<'a> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute<'a> + Clone,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IElement<'a> + Clone,
    &'a IElement<'a>: From<TChildren>,
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

impl<'a, TAttributes: 'a, TChildren: 'a> From<&'a IHtmlElement<'a>> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IAttribute<'a> + Clone,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IElement<'a> + Clone,
    &'a IElement<'a>: From<TChildren>,
{
    fn from(src: &'a IHtmlElement) -> Element<'a, TAttributes, TChildren> {
        Element {
            kind: src.kind(),
            attributes: &[],
            children: &[],
        }
    }
}

#[derive(Clone, Debug)]
pub struct HtmlElement<'a, TAttributes: 'a, TChildren: 'a>
where
{
    pub kind: &'a str,
    pub attributes: &'a [TAttributes],
    pub children: &'a [TChildren],
}

impl<'a, TAttributes: 'a, TChildren: 'a> HtmlElement<'a, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute<'a> + IAttribute<'a> + Clone,
    &'a IHtmlAttribute<'a>: From<TAttributes>,
    TChildren: IHtmlElement<'a> + IElement<'a> + Clone,
    &'a IHtmlElement<'a>: From<TChildren>,
{
    pub fn new(attributes: &'a [TAttributes], children: &'a [TChildren]) -> Self {
        HtmlElement {
            kind: "html",
            attributes,
            children,
        }
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> From<&'a HtmlElement<'a, TAttributes, TChildren>> for Element<'a, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute<'a> + IAttribute<'a> + Clone,
    &'a IHtmlAttribute<'a>: From<TAttributes>,
    &'a IAttribute<'a>: From<TAttributes>,
    TChildren: IHtmlElement<'a> + IElement<'a> + Clone,
    &'a IHtmlElement<'a>: From<TChildren>,
    &'a IElement<'a>: From<TChildren>,
{
    fn from(src: &'a HtmlElement<'a, TAttributes, TChildren>) -> Element<'a, TAttributes, TChildren> {
        Element::new(src.kind, src.attributes, src.children)
    }
}

impl<'a, TAttributes: 'a, TChildren: 'a> IElement<'a> for HtmlElement<'a, TAttributes, TChildren>
where
    TAttributes: IHtmlAttribute<'a> + IAttribute<'a> + Clone,
    &'a IHtmlAttribute<'a>: From<TAttributes>,
    TChildren: IHtmlElement<'a> + IElement<'a> + Clone,
    &'a IHtmlElement<'a>: From<TChildren>,
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