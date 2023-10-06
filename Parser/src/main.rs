use latex::{DocumentClass, Element, Document, Section, Align};

fn main() 
{
    let mut doc = Document::new(DocumentClass::Article);

    doc.preamble.title("CV");
    dc.preamble.author("Name");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage);

    let mut section_1 = Section::new("Section1");
    section_1.push("Some text sfasgsaassg hfjhdf")
            .push("safgdsgsdg fsafsa fsasfasf");
    doc.push(section_1);

    let mut section_2 = Section::new("Section2");

    section_2.push("more text xvdsgds")
        .push(Align::from("y &= mx + c"));

    doc.push(section_2);

    let rendered = latex::prin(&doc)?;

}
