

pub trait IElement {
    type TAttributes;
}

#[derive(Debug)]
pub struct Element<TAttribute> {
    pub kind: String,
    pub attributes: Vec<TAttribute>,
}

#[derive(Debug)]
pub struct Html<'a>(Element<&'a IHtmlAttribute>);

pub trait IHtmlAttribute: std::fmt::Debug {}

impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

// impl<'a, T> IElement for Html<'a>
// where
//     T: IHtmlAttribute + Clone,
// {
//     type TAttributes = T;
// }

#[derive(Clone, Debug)]
pub struct Foo {}

#[derive(Clone, Debug)]
pub struct Bar {}

impl IHtmlAttribute for Foo {}

impl IHtmlAttribute for Bar {}

pub fn html<'a, 'b: 'a>(attributes: &'b [&IHtmlAttribute]) -> Html<'b>
{
    Html(
        Element::<&IHtmlAttribute>{
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
        let element = html(&[&Foo {}, &Bar {}]);

        println!("{:?}", element);

        assert!(false);
    }
}