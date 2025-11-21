const CryptoJS = require("crypto-js");
const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener('open', () => {
    console.log("Connected to database server");
});

socket.addEventListener('message', (event) => {
    try{
        const data = event.data;
        if (data.compare("-1")){
            console.log("Wrong E-mail and/or Password");
        } else {
            localStorage.setItem("UserRole", data)
        }
    } catch (error){
        console.error("ERROR parsing message from server:", error);
    }
    console.log("Message from server:", event.data);
});

//get string input
const loginForm = document.getElementById('login');
const emailInput = document.getElementById('emailInput');
const pwdInput = document.getElementById('pwdInput');

loginForm.addEventListener('submit', (e) => {
    // stop page reload
    // e.preventDefault(); 

    // get the login input
    const emailString = emailInput.value;
    const hashedPWDString = CryptoJS.SHA256(pwdInput.value).toString();

    //build JSON
    const message = {
        email: emailString,
        password: hashedPWDString
    };

    //send to websocket server
    if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify(message));
        console.log("Sent:", message);
    }
});
