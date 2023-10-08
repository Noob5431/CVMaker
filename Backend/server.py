from gevent import monkey;  monkey.patch_all()
from flask import Flask, Response, render_template, stream_with_context, send_file, request
from gevent.pywsgi import WSGIServer
from flask_cors import CORS, cross_origin
from flask_sse import sse
import json
import time

##########################################
#       change routes on everythingh     #
##########################################
app =Flask(__name__)

i=0
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'
app.register_blueprint(sse , url_prefix = "/stream")
class Cv_data:
    def __init__(self):
        
        self.Name = "what is your name"
        self.PhoneNumber = "what is your phone number"
        self.City = "in what city do you live"
        self.EmailAddress = "in what city do you live"
        
        #INDEX DE vector
        self.UniversityCount = "do you have a masters degree"
        self.UniversityName = []
#        self.UniversityName.append("what university have you attended or are attendindg")
        self.FieldOfStudy = []
#       self.FieldOfStudy.append("what are you studying or have studied")
        self.StartDate = []
#        self.StartDate.append("in what year did you start your studies")
        self.EndDate = []
  #      self.EndDate.append("when have you finished your studies or what is your expected graduation year")
        
        self.TechnicalSkills = "what technical skills do you have"
        self.SoftSkills = "what soft skills do you have"
        
        #INDEX DE vector
        self.WorkCount = "how many jobs have you had"
        self.Workplace = []
    #    self.Workplace.append("where did you work at")
        self.WorkCity = []
  #      self.WorkCity.append("in what city did you work at")
        self.WorkStart = []
 #       self.WorkStart.append("when did you start working at this company")
        self.WorkEnd = []
    #    self.WorkEnd.append("when did you end working at this company")
        self.RoleName = []
  #      self.RoleName.append("what was your role")
        self.WorkDescription = []
   #     self.WorkDescription.append("describe the work you did")
        
        #INDEX DE vector
        self.ProjectCount = "how many projects do you want to add to your cv"
        self.ProjectName = []
 #       self.ProjectName.append("what is the name of your project")
        self.ProjectDescription = []
  #      self.ProjectDescription.append("describe the work you did on the project")
        
        #INDEX DE vector
        self.ExtraActivityCount = "how many extracurricular activities do you want to add to your cv"
        self.ExtraActivity = []
  #      self.ExtraActivity.append("describe an extracurricular activity")


element_names=["Name",
"PhoneNumber",
"City",
"EmailAddress",
"UniversityCount",
"UniversityName",
"FieldOfStudy",
"StartDate",
"EndDate",
"TechnicalSkills",
"SoftSkills",
"WorkCount",
"Workplace",
"WorkCity",
"WorkStart",
"WorkEnd",
"RoleName",
"WorkDescription",
"ProjectCount",
"ProjectName",
"ProjectDescription",
"ExtraActivityCount",
"ExtraActivity"]

cv_data=Cv_data()
lock=False
string_question=["what is your name","what is your phone number","whats your email adress","do you have a masters degree","what university have you attended or are attendindg","what are you studying or have studied","in what year did you start your studies"
                 "when have you finished your studies or what is your expected graduation year","what technical skills do you have","what soft skills do you have","how many jobs have you had","where did you work at","in what city did you work at","when did you start working at this company"
                 ,"when did you end working at this company","what was your role","describe the work you did","how many projects do you want to add to your cv","what is the name of your project", "describe the work you did on the project",  "how many extracurricular activities do you want to add to your cv","describe an extracurricular activity"]
lampj=0
@app.route("/")
def star_site():
    resp = Flask.Response("Foo bar baz")
    resp.headers['Access-Control-Allow-Origin'] = '*'
    resp.headers['Access-Control-Allow-Headers'] = '*'
    return render_template("site.html")



@app.route("/sse")
def sse():
    global lock
    global string_question
    def event_stream():
        global lock
        global lampj
        if lock==True:
            yield f"data: {string_question[lampj]}\n\n"
            lampj=lampj+1
            lock=False
    return Response(event_stream(),content_type= "text/event-stream")

#get answers from user
@app.route('/', methods=['POST'])
@cross_origin(origin='*')
def userVomit():
    if request.method == 'POST':
        global i
        global lock
        global string_question
        if i < len(element_names):
            for x in request.form :
                if(isinstance(getattr(cv_data,element_names[i]), list) ):
                   
                    getattr(cv_data,element_names[i]).append(x)
                else:
                    setattr(cv_data,element_names[i], x )
                i=i+1
                lock=True
                print(i)
        else:
            export_json()
    return  '1'

def export_json():
    #modify fbard
    for j in cv_data.WorkDescription:
        cv_data.WorkDescription[j]=fBard(0,cv_data.WorkDescription[j])
    
    for j in cv_data.ProjectDescription:
        cv_data.ProjectDescription[j]=fbard(1,cv_data.ProjectDescription[j])
    
    for j in cv_data.ExtraActivity:
        cv_data.ExtraActivity[j]=fbard(2,cv_data.ExtraActivity[j])
    
    cv_output=json.dumps(cv_data.__dict__)
    f= open("CV.json","w")
    f.write(cv_output)
    f.close()

if __name__ == "__main__":
  # app.run(port=80, debug=True)
  http_server = WSGIServer(("localhost", 5000), app)
  http_server.serve_forever()