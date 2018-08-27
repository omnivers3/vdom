

pub trait IElement {
    type TAttributes;
}

#[derive(Debug)]
pub struct Element<TAttribute> {
    pub attribute: TAttribute,
}

#[derive(Debug)]
pub struct Html<TAttribute>(Element<TAttribute>);

pub trait IHtmlAttribute {}

impl<TAttribute> IElement for Html<TAttribute>
where
    TAttribute: IHtmlAttribute + Clone,
{
    type TAttributes = TAttribute;
}

#[derive(Clone, Debug)]
pub struct Foo {}

impl IHtmlAttribute for Foo {}

pub fn html<TAttribute>(attribute: TAttribute) -> Html<TAttribute>
where
    TAttribute: IHtmlAttribute + Clone,
{
    Html::<TAttribute>(
        Element::<TAttribute>{
            attribute,
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should() {
        let element = html(Foo {});

        println!("{:?}", element);

        assert!(false);
    }
}