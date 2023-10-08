use std::fs::{File, self};
use std::io::Write;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Packet
{
    Name : String,
    PhoneNumber : String,
    City : String,
    EmailAddress : String,
    UniversityCount : String,
    UniversityName : Vec<String>,
    FieldOfStudy : Vec<String>,
    StartDate : Vec<String>,
    EndDate : Vec<String>,
    TechnicalSkills : String,
    SoftSkills : String,
    WorkCount : String,
    Workplace : Vec<String>,
    WorkCity : Vec<String>,
    WorkStart : Vec<String>,
    WorkEnd : Vec<String>,
    RoleName : Vec<String>,
    WorkDescription : Vec<String>,
    ProjectCount : String,
    ProjectName : Vec<String>,
    ProjectDescription : Vec<String>,
    ExtraActivityCount : String,
    ExtraActivity : Vec<String>,
}

fn main() 
{
    let mut latex_code : String = "".to_string();
    //user variables
    let mut field_of_study : String = "Computer Science".to_string();
    let mut university_name : String = "UPB".to_string();
    let mut start_date : String = "2022".to_string();
    let mut end_date : String = "2026".to_string();
    let mut role_name : String = "Software Developer".to_string();
    let mut workplace : String = "Google".to_string();
    let mut workcity : String = "Bucharest".to_string();
    let mut work_description : String = "default asifjasljwil ijfaw ghdb iejwopij sjfslj lsfjaifi lsjfalifj wjli j lfsjf iwadjlawj sfsjfi wjoajfi dijfdij waij".to_string();
    let mut work_start : String = "2022".to_string();
    let mut work_end : String = "2023".to_string();
    let mut project_name : String = "Project1".to_string();
    let mut project_description : String = "default  owirhqoi hgiodshgeoh io oehwtioewhig heiogthwow hqwoihroqirh paojfo".to_string();
    let mut extra_activity : String = "Write blog posts about best practices".to_string();
    //read json packet
    let json_path : String = "C:\\Users\\andre\\Desktop\\RO2030\\CVMaker\\Parser\\JSON_test.txt".to_string();
    let json_code = fs::read_to_string(json_path).expect("could not read file");

    let packet : Packet = serde_json::from_str(&json_code).unwrap();

    //generate latex code
    latex_code.push_str(&("\\documentclass{resume}
        \\usepackage[left=0.4 in,top=0.4in,right=0.4 in,bottom=0.4in]{geometry}
        \\newcommand{\\tab}[1]{\\hspace{.2667\\textwidth}\\rlap{#1}}
        \\newcommand{\\itab}[1]{\\hspace{0em}\\rlap{#1}}
        \\name{".to_string() + &packet.Name + "}
        \\address{" + &packet.PhoneNumber + " \\\\ " + &packet.City + "}
        \\address{\\href{mailto:" + &packet.EmailAddress + "}{" + &packet.EmailAddress + "}}
        \\begin{document}
        \\begin{rSection}{Education}"));

        //iterate over all studies
        for i in 0..packet.FieldOfStudy.len() 
        {
            field_of_study = packet.FieldOfStudy.get(i).unwrap().to_string();
            university_name = packet.UniversityName.get(i).unwrap().to_string();
            start_date = packet.StartDate.get(i).unwrap().to_string();
            end_date = packet.EndDate.get(i).unwrap().to_string();

            latex_code.push_str(&("{\\bf ".to_string() + &field_of_study + "}, " + &university_name + "\\hfill {" + &start_date + " - " + &end_date + "}"));
            if i<packet.FieldOfStudy.len()-1
            {
                latex_code.push_str("\\\\");
            }
        }
        //{\\bf " + &field_of_study + "}, " + &university_name + "\\hfill {" + &start_date + " - " + &end_date + "}
        latex_code.push_str(
        &("\\end{rSection}
        \\begin{rSection}{SKILLS}
        \\begin{tabular}{ @{} >{\\bfseries}l @{\\hspace{6ex}} l }
        Technical Skills & ".to_string() + &packet.TechnicalSkills + "\\\\
        Soft Skills & " + &packet.SoftSkills + "
        \\end{tabular}\\\\
        \\end{rSection}
        \\begin{rSection}{EXPERIENCE}"));

        //iterate through work experience
        for i in 0..packet.WorkStart.len()
        {
            role_name = packet.RoleName.get(i).unwrap().to_string();
            work_start = packet.WorkStart.get(i).unwrap().to_string();
            work_end = packet.WorkEnd.get(i).unwrap().to_string();
            workplace = packet.Workplace.get(i).unwrap().to_string();
            workcity = packet.WorkCity.get(i).unwrap().to_string();

            latex_code.push_str(&(
                "\\textbf{".to_string() + &role_name + "} \\hfill " + &work_start + " - " + &work_end + "{\\\\
                " + &workplace + " \\hfill \\textit{" + &workcity + "}
                \\begin{itemize}
                \\itemsep -3pt {}
                \\item " + &work_description + "
                \\end{itemize}"
            ))
        }

        //\\textbf{" + &role_name + "} \\hfill " + &work_start + " - " + &work_end + "{\\\\
        //" + &workplace + " \\hfill \\textit{" + &workcity + "}
        //\\begin{itemize}
        //\\itemsep -3pt {}
        //\\item " + &work_description + "
        //\\end{itemize}

        latex_code.push_str(&(
        "\\end{rSection}
        \\begin{rSection}{PROJECTS}
        \\vspace{-1.25em}"));
        
        //iterate through projects
        for i in 0..packet.ProjectName.len()
        {
            project_name = packet.ProjectName.get(i).unwrap().to_string();
            project_description = packet.ProjectDescription.get(i).unwrap().to_string();

            latex_code.push_str(&(
                "\\item \\textbf{".to_string() + &project_name + "} {" + &project_description + "}"
            ));
        }
        //\\item \\textbf{".to_string() + &project_name + "} {" + &project_description + "}
        
        latex_code.push_str(&(
        "\\end{rSection}
        \\begin{rSection}{Extra-Curricular Activities} 
        \\begin{itemize}"));
        
        for i in 0..packet.ExtraActivity.len()
        {
            extra_activity = packet.ExtraActivity.get(i).unwrap().to_string();

            latex_code.push_str(&("\\item ".to_string() + &extra_activity));
        }
        //\\item ".to_string() + &extra_activity + "
        latex_code.push_str(&(
        "
        \\end{itemize}
        \\end{rSection}
        \\end{document}"));
    
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
