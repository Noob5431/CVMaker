from flask import Flask, request, jsonify

#from .ChatbotAPI.another_script import process_name

app = Flask(__name__)


@app.route('/process',method=['POST'])

def addToCV(data):
    print("works")
    #if CV dosent exist create
    #else add to CV
    #based on data add to correct prompt

def process():
    unprocessd_data=request.get_json()

    #send data to chat chose the correct one
    #
    #process_name(unprocessd_data)
    #data=process_name(unprocessd_data)
    #

    #shove data into cv based on the type
    #addToCV(data)
    
    #ask next question 
    next_question=getQuestion()

    #send next question or end chat
    return jsonify(next_question)



if __name__ == '__main__':
    app.run()