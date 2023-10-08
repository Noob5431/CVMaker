document.addEventListener("DOMContentLoaded", function () {
    const chatContainer = document.querySelector(".chat-container");
    const inputText = document.getElementById("input-text");
    const sendButton = document.getElementById("send-button");
    const chatMessages = document.getElementById("chat-messages");

    document.addEventListener('keyup', function(event) {
        if (event.code === 'Enter')
        {
            event.preventDefault();
            document.getElementById("send-button").click();
        }
    });

    sendButton.addEventListener("click", function () {
        const userMessage = inputText.value.trim();
        

        if (userMessage === "") {
            return;
        }

        appendMessage("user", userMessage);
        inputText.value = ""; // Clear the input field

        // Simulate a bot response (replace with your actual bot logic)
        //simulateBotResponse(userMessage);

        let xhr = new XMLHttpRequest();
        let url = "http://localhost:5000";

        xhr.open("POST", url, true);

        //xhr.setRequestHeader("Access-Control-Allow-Origin", "*");

        xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");

        xhr.send(userMessage);
    });

    var source = new EventSource("http://localhost:5000/sse");
    source.onmessage = function eventListener(event) {
        simulateBotResponse(event.data);
    }; 

    function appendMessage(sender, message) {
        const messageContainer = document.createElement("div");
        messageContainer.classList.add("message", `${sender}-message`);
        messageContainer.textContent = message;
        chatMessages.appendChild(messageContainer);
        chatContainer.scrollTop = chatContainer.scrollHeight; // Scroll to the bottom of the chat
    }

    function simulateBotResponse(userMessage) {
        // Simulate a bot response (replace with your actual bot logic)
        const botResponse = generateBotResponse(userMessage);
        appendMessage("bot", botResponse);
    }

    function generateBotResponse(userMessage) {
        // This is where you can implement your chatbot's logic.
        // For now, let's echo back the user's message.
        return "Bot: You said: " + userMessage;
    }
});
