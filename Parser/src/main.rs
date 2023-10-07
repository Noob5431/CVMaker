use latex::{DocumentClass, Element, Document, Section, Align, PreambleElement};
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() 
{
    let mut latex_code : String = "".to_string();
    //user variables
    let mut name : String = "Default".to_string();
    let mut phone_number : String = "123456789".to_string();
    let mut city : String = "San Francisco".to_string();
    let mut email_address : String = "name@email.com".to_string();
    let mut field_of_study : String = "Computer Science".to_string();
    let mut university_name : String = "UPB".to_string();
    let mut start_date : String = "2022".to_string();
    let mut end_date : String = "2026".to_string();
    let mut skills : String = "C, C++, Rust, Python".to_string();
    let mut soft_skills : String = "Leadership, Communication".to_string();
    let mut role_name : String = "Software Developer".to_string();
    let mut workplace : String = "Google".to_string();
    let mut workcity : String = "Bucharest".to_string();
    let mut work_description : String = "asifjasljwil ijfaw ghdb iejwopij sjfslj lsfjaifi lsjfalifj wjli j lfsjf iwadjlawj sfsjfi wjoajfi dijfdij waij".to_string();
    let mut work_start : String = "2022".to_string();
    let mut work_end : String = "2023".to_string();
    let mut project_name : String = "Project1".to_string();
    let mut project_description : String = "sajfpasj pjsdpogj etjeiotj haiohioahfs hjweiotehoig ihoasfiohafoi eiotheioh klsnalfka wlhliqrhior hioahfgsoidg tehwioth fdshogsdhw owirhqoi hgiodshgeoh io oehwtioewhig heiogthwow hqwoihroqirh paojfo".to_string();
    let mut extra_activity : String = "Write blog posts about best practices".to_string();


    //generate latex code
    latex_code.push_str(&("\\documentclass{resume}
        \\usepackage[left=0.4 in,top=0.4in,right=0.4 in,bottom=0.4in]{geometry}
        \\newcommand{\\tab}[1]{\\hspace{.2667\\textwidth}\\rlap{#1}}
        \\newcommand{\\itab}[1]{\\hspace{0em}\\rlap{#1}}
        \\name{".to_string() + &name + "}
        \\address{" + &phone_number + " \\\\ " + &city + "}
        \\address{\\href{mailto:" + &email_address + "}{" + &email_address + "}}
        \\begin{document}
        \\begin{rSection}{Education}
        {\\bf " + &field_of_study + "}, " + &university_name + "\\hfil {" + &start_date + " - " + &end_date + "}
        \\end{rSection}
        \\begin{rSection}{SKILLS}
        \\begin{tabular}{ @{} >{\\bfseries}l @{\\hspace{6ex}} l }
        Technical Skills & " + &skills + "\\\\
        Soft Skills & " + &soft_skills + "
        \\end{tabular}\\\\
        \\end{rSection}
        \\begin{rSection}{EXPERIENCE}
        \\textbf{" + &role_name + "} \\hfill " + &work_start + " - " + &work_end + "{\\\\
        " + &workplace + " \\hfill \\textit{" + &workcity + "}
        \\begin{itemize}
        \\itemsep -3pt {}
        \\item " + &work_description + "
        \\end{itemize}
        \\end{rSection}
        \\begin{rSection}{PROJECTS}
        \\vspace{-1.25em}
        \\item \\textbf{" + &project_name + "} {" + &project_description + "}
        \\end{rSection}
        \\begin{rSection}{Extra-Curricular Activities} 
        \\begin{itemize}
        \\item " + &extra_activity + "
        \\end{itemize}
        \\end{rSection}
        \\end{document}"));
    //latex formatting
    /*
    let mut doc = Document::new(DocumentClass::Other("resume".to_string()));

    //preamble
    doc.preamble.push(PreambleElement::UserDefined
        (("\\usepackage[left=0.4 in,top=0.4in,right=0.4 in,bottom=0.4in]{geometry}")
        .to_string()));
    doc.preamble.push(PreambleElement::UserDefined
        (("\\newcommand{\\tab}[1]{\\hspace{.2667\\textwidth}\\rlap{#1}}")
        .to_string()));
    doc.preamble.push(PreambleElement::UserDefined
        (("\\newcommand{\\itab}[1]{\\hspace{0em}\\rlap{#1}}")
        .to_string()));
    doc.preamble.push(PreambleElement::UserDefined
        (("\\name{".to_string() + &name + "}")
        .to_string()));
    doc.preamble.push(PreambleElement::UserDefined
        (("\\address{".to_string() + &phone_number + " \\\\ " + &city + "}")
        .to_string()));
    doc.preamble.push(PreambleElement::UserDefined
        (("\\address{\\href{mailto:".to_string() + &email_address + "}{" + 
        &email_address + "}}")
        .to_string()));
    
    //document start
    doc.push("\\begin{rSection}{Education}");
    doc.push(Element::UserDefined("{\\bf".to_string() + &field_of_study + "}, " 
    + &university_name + "\\hfill {" + &start_date + " - " + &end_date));
    doc.push(Element::UserDefined("\\end{rSection".to_string()));

    doc.push(Element::UserDefined("\\begin{rSection}{SKILLS".to_string()));
    doc.push(Element::UserDefined("\\begin{tabular}{ @{} >{\\bfseries}l 
    @{\\hspace{6ex}} l }".to_string()));
    doc.push(Element::UserDefined("Technical Skills & ".to_string() + &skills));
    doc.push(Element::UserDefined("Soft Skills & ".to_string() + &soft_skills));
    doc.push(Element::UserDefined("\\end{tabular}\\\\".to_string()));
    doc.push(Element::UserDefined("\\end{rSection}".to_string()));
    doc.push(Element::UserDefined("\\begin{rSection}{EXPERIENCE}".to_string()));
        */
    //get latex code as string
    //let rendered = latex::print(&doc).unwrap();

    //get path of created tex file
    let mut source_path = project_root::get_project_root().unwrap();
    source_path.push("MikTex");
    source_path.push("CV.tex");

    //create tex file
    let mut f = File::create(source_path.clone()).unwrap();
    write!(f,"{}",latex_code).unwrap();
    
    //get path to tex compiler
    let mut path = project_root::get_project_root().unwrap();
    path.push("MikTex/texmfs/install/miktex/bin/x64/pdflatex.exe");

    //get path to created pdf file
    let output_path_helper = String::from("--output-directory=");
    let output_path = output_path_helper + &project_root::get_project_root().unwrap().into_os_string().into_string().unwrap();
    //println!("{}", output_path);
    //compile tex to pdf
    let exit_status = Command::new(path)
    .arg(source_path.clone()).
    arg(output_path).status().unwrap();

    //std::fs::remove_file(source_path).unwrap();
    assert!(exit_status.success());
}
