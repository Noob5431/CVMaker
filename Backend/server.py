from gevent import monkey;  monkey.patch_all()
from flask import Flask, Response, render_template, stream_with_context, send_file, request
from gevent.pywsgi import WSGIServer
from flask_cors import CORS, cross_origin
import json
import time

##########################################
#       change routes on everythingh     #
##########################################
app =Flask(__name__)

i=0
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

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
print(len(element_names))

@app.route("/")
def star_site():
    resp = Flask.Response("Foo bar baz")
    resp.headers['Access-Control-Allow-Origin'] = '*'
    resp.headers['Access-Control-Allow-Headers'] = '*'
    return render_template("site.html")


@app.route("/")
def listen(questions):
    def respondToClients(questions):
        for attr_name, attr_value in vars(questions).items():
            curr_quest = getattr(questions, attr_name)
            #sends curr_quest to eveniment quest
            print(curr_quest)
            yield f"id: 1\ndata: {curr_quest}\nevent: quest\n\n"
            time.sleep(1)
    return Response(respondToClients(), mimetype= 'text/event-stream')


#get answers from user
@app.route('/', methods=['POST'])
@cross_origin(origin='*')
def userVomit():
    if request.method == 'POST':
        global i
        if i < len(element_names):
            for x in request.form :
                if(isinstance(getattr(cv_data,element_names[i]), list) ):
                    print("facut")
                    getattr(cv_data,element_names[i]).append(x)
                else:
                    setattr(cv_data,element_names[i], x )
                i=i+1
                print(i)
        else:
            export_json()
    return  '1'

def export_json():
    cv_output=json.dumps(cv_data.__dict__)
    f= open("CV.json","w")
    f.write(cv_output)
    f.close()

if __name__ == "__main__":
  # app.run(port=80, debug=True)
  http_server = WSGIServer(("localhost", 5000), app)
  http_server.serve_forever()