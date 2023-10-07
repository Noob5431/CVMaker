from flask import Flask, request, jsonify, render_template

#change path to the site folder  if needed ( site folder )
app = Flask(__name__, website='../Website')

#access site 
@app.route('/')
def main_page():
    #change file name
    #change return type if not static for the html
    return render_template("site.html")        

# /questionRequest is the page you want to insert to change with path
@app.route('/questionRequest', method=['GET'])
def questionRequest():
    #question function
    question="string"
    return render_template("questionRequest.html",question)


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

if __name__ == '__main__':
    app.run()
    
    
    