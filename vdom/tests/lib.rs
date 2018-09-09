extern crate vdom;

use vdom::*;

#[test]
fn should_cast_to_root_interface() {
    let _: &IElement = &Element::new("thing", &[], &[]);
    let _: &IElement = &HtmlElement::new(&[], &[]);
    let _: &IElement = &BodyElement::new(&[], &[]);

    // assert!(false);
}

#[test]
fn should_allow_base_element_in_more_specific_cases() {
    let element = Element::new("thing", &[], &[]);
    let html_children: Vec<&IHtmlElement> = vec![&element];
    let html_element = HtmlElement::new(&[], &html_children);
    // let e2: &IHtmlElement = &element;
    // let html_element = HtmlElement::new(&[], vec![e2].as_slice());

    assert_eq!(1, html_element.children.len());
    // let t: Debug = &element;
    // println!("{:?}", t);
    let child: ElementKinds = html_element.children[0].into();
    println!("Html: {:?}", child);
    println!("{:?}", html_element.children[0]);
    assert_eq!(element.kind, html_element.children[0], "fail");
    println!("{:?}", html_element);

    assert!(false);
}

#[test]
fn should_be_able_to_coerce_invalid_elements_if_desired() {
    let html_invalid = HtmlElement::new(&[], &[]);
    // This will not compile due to HtmlElement missing required IBodyElement trait
    // let body_children: Vec<&IBodyElement> = vec![html_invalid];
    // But this will after coercing the HtmlElement into an IElement
    let html_valid: &IElement = &html_invalid;
    let body_children: Vec<&IBodyElement> = vec![&html_valid];
    let body = &BodyElement::new(&[], &body_children);

    // println!("{:?}", body);

    // assert!(false);
}

#[test]
fn should_build_html_with_valid_children() {
    let element_1 = Element::new("thing", &[], &[]);
    let html_invalid = HtmlElement::new(&[], &[]);
    let html_valid: &IElement = &html_invalid;
    let body_children: Vec<&IBodyElement> = vec![&element_1, &html_valid];
    let body_1 = BodyElement::new(&[], &body_children);
    let html_children: Vec<&IHtmlElement> = vec![&element_1, &body_1];
    let html = &HtmlElement::new(&[], &html_children);

    // println!("{:?}", html);

    // assert!(false);
}

#[test]
fn should_build_element() {
    let element1 = Element::new("element-a", &[], &[]);
    let element2_children: Vec<&IElement> = vec![&element1];
    let element2 = Element::new("element-b", &[], &element2_children);
    let element3_great_grand_children: Vec<&IElement> = vec![];
    let element3_grand_child = Element::new("element-c-2-1", &[], &element3_great_grand_children);
    let element3_children: Vec<&IElement> = vec![&element1, &element3_grand_child];
    {
        let element3 = Element::new("element-c", &[], &element3_children);
    }

    // println!("{:?}", element1);
    // println!("{:?}", element2);
    // println!("{:?}", element3);

    // assert!(false);
}
