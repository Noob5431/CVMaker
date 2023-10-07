from flask import Flask, request, jsonify, render_template

import os
template_dir = os.path.dirname(os.path.dirname(os.path.abspath(os.path.dirname(__file__))))
template_dir = os.path.join(template_dir, 'website')

app = Flask(__name__, template_folder=template_dir)



#change path to the site folder  if needed ( site folder ) if the whole os thingh no good
#app = Flask(__name__, website='../Website')

#access site 
@app.route('/')
def main_page():
    #change file name
    #change return type if not static for the html
    return render_template("site.html")        

# /questionRequest is the page you want to insert to change with path
@app.route('/questionRequest', method=['POST'])
class Packet:
    def __init__(self):
        self.Name = "what is your name"
        self.PhoneNumber = "what is your phone number"
        self.City = "in what city do you live"
        self.EmailAddress = "in what city do you live"
        
        #INDEX DE vector
        self.UniversityCount = "do you have a masters degree"
        self.UniversityName = []
        self.UniversityName.append("what university have you attended or are attendindg")
        self.FieldOfStudy = []
        self.FieldOfStudy.append("what are you studying or have studied")
        self.StartDate = []
        self.StartDate.append("in what year did you start your studies")
        self.EndDate = []
        self.EndDate.append("when have you finished your studies or what is your expected graduation year")
        
        self.TechnicalSkills = "what technical skills do you have"
        self.SoftSkills = "what soft skills do you have"
        
        #INDEX DE vector
        self.WorkCount = "how many jobs have you had"
        self.Workplace = []
        self.Workplace.append("where did you work at")
        self.WorkCity = []
        self.WorkCity.append("in what city did you work at")
        self.WorkStart = []
        self.WorkStart.append("when did you start working at this company")
        self.WorkEnd = []
        self.WorkEnd.append("when did you end working at this company")
        self.RoleName = []
        self.RoleName.append("what was your role")
        self.WorkDescription = []
        self.WorkDescription.append("describe the work you did")
        
        #INDEX DE vector
        self.ProjectCount = "how many projects do you want to add to your cv"
        self.ProjectName = []
        self.ProjectName.append("what is the name of your project")
        self.ProjectDescription = []
        self.ProjectDescription.append("describe the work you did on the project")
        
        #INDEX DE vector
        self.ExtraActivityCount = "how many extracurricular activities do you want to add to your cv"
        self.ExtraActivity = []
        self.ExtraActivity.append("describe an extracurricular activity")
        

# Usage example:
packet = Packet()

def addToCV(data):
    print("works")
    #if CV dosent exist create
    #else add to CV
    #based on data add to correct prompt

# /userVommit is the page you want to extract from change with path
@app.route('/userVommit' , method=['POST'])
def userVommit():
    unprocessd_data=request.get_json()

    #send data to chat ( change name as needed )
    #process_name(unprocessd_data)
    data=bardFunc(unprocessd_data)
    

    addToCV(data)
    
    #ask next question prob not needed
    #next_question=getQuestion()

    #send next question or end chat
    """
    if next_question!= '\n' :
        return jsonify(next_question)
    else:
        print('send pdf')
    """
    return render_template("questionRequest.html")

if __name__ == '__main__':
    app.run()
    
    
    