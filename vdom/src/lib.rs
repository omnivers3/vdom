

pub trait IElement {
    type TAttributes;
}

#[derive(Clone, Debug)]
pub struct Element<TAttribute> {
    pub kind: String,
    pub attributes: Vec<TAttribute>,
}

pub trait IHtmlAttribute: std::fmt::Debug {}

impl<'a> IHtmlAttribute for &'a IHtmlAttribute {}

#[derive(Clone, Debug)]
pub struct Html<T>(Element<T>) where T: IHtmlAttribute + Clone;
// pub struct Html<'a>(Element<&'a IHtmlAttribute>);

pub trait IBodyAttribute: std::fmt::Debug {}

impl<'a> IBodyAttribute for &'a IBodyAttribute {}

#[derive(Clone, Debug)]
pub struct Body<T>(Element<T>) where T: IBodyAttribute + Clone;

impl<T> IHtmlAttribute for Body<T> where T: IBodyAttribute + Clone {}

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

#[derive(Clone, Debug)]
pub struct Baz {}

impl IHtmlAttribute for Foo {}

impl IHtmlAttribute for Bar {}

impl IBodyAttribute for Bar {}

impl IBodyAttribute for Baz {}

// pub fn html<'a, 'b: 'a, T: 'b>(attributes: &'b [&IHtmlAttribute]) -> Html<T>
pub fn html<'a, 'b: 'a, T: 'b>(attributes: &'a [T]) -> Html<T>
where
    T: IHtmlAttribute + Clone,
    &'b IHtmlAttribute: From<T>,
    // &'b T: IHtmlAttribute + Clone,
    // IHtmlAttribute: From<T>,
{
    Html(
        Element::<T>{
            kind: "html".to_owned(),
            attributes: attributes.to_vec(),
            // attributes: attributes.to_vec().into_iter().map(|item| item.into()).collect(),
        }
    )
}

pub fn body<'a, T: 'a>(attributes: &'a [T]) -> Body<T>
where
    T: IBodyAttribute + Clone,
    &'a IBodyAttribute: From<T>,
{
    Body(
        Element::<T>{
            kind: "body".to_owned(),
            attributes: attributes.to_vec(),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should() {
        // let html1 = html(&[&Foo {}, &Bar {}, &Bar {}]);
        let body1 = body(&[&Bar {}, &Baz {}]);

        let html2 = html(
            &[ &body1
            ]
        );

        // println!("{:?}", html1);
        println!("{:?}", body1);

        println!("{:?}", html2);

        assert!(false);
    }
}