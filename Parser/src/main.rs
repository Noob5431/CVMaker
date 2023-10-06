use latex::{DocumentClass, Element, Document, Section, Align};
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() 
{
    let mut doc = Document::new(DocumentClass::Article);

    doc.preamble.title("CV");
    doc.preamble.author("Name");

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

    let rendered = latex::print(&doc).unwrap();

    let mut source_path = project_root::get_project_root().unwrap();
    source_path.push("MikTex");
    source_path.push("CV.tex");
    let mut f = File::create(source_path.clone()).unwrap();
    write!(f,"{}",rendered).unwrap();

    let mut path = project_root::get_project_root().unwrap();
    path.push("MikTex/texmfs/install/miktex/bin/x64/pdftex.exe");

    let output_path_helper = String::from("--output-directory==");
    let output_path = output_path_helper + &project_root::get_project_root().unwrap().into_os_string().into_string().unwrap();
    let exit_status = Command::new(path)
    .arg(source_path.clone()).
    arg(output_path).status().unwrap();


    std::fs::remove_file(source_path).unwrap();
    assert!(exit_status.success());
}
