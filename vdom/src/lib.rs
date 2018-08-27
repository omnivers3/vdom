

pub trait IElement {
    type TAttributes;
}

#[derive(Debug)]
pub struct Element<TAttribute> {
    pub kind: String,
    pub attributes: Vec<TAttribute>,
}

#[derive(Debug)]
pub struct Html<TAttribute>(Element<TAttribute>);

pub trait IHtmlAttribute: std::fmt::Debug {}

impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

impl<TAttribute> IElement for Html<TAttribute>
where
    TAttribute: IHtmlAttribute + Clone,
{
    type TAttributes = TAttribute;
}

#[derive(Clone, Debug)]
pub struct Foo {}

#[derive(Clone, Debug)]
pub struct Bar {}

impl IHtmlAttribute for Foo {}

impl IHtmlAttribute for Bar {}

pub fn html<TAttribute>(attributes: &[TAttribute]) -> Html<TAttribute>
where
    TAttribute: IHtmlAttribute + Clone + std::fmt::Debug,
{
    Html::<TAttribute>(
        Element::<TAttribute>{
            kind: "html".to_owned(),
            attributes: attributes.to_vec(),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should() {
        let attr1: &IHtmlAttribute = &Foo {};
        let attr2: &IHtmlAttribute = &Bar {};

        let element = html(&[attr1, attr2]);

        println!("{:?}", element);

        assert!(false);
    }
}