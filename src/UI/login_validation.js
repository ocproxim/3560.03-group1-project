const CryptoJS = require("crypto-js");
const socket = new WebSocket("ws://localhost:8080");

//get string input
const loginForm = document.getElementById('login');
const emailInput = document.getElementById('emailInput');
const pwdInput = document.getElementById('pwdInput');

socket.addEventListener('open', () => {
    console.log("Connected to database server");
});

socket.addEventListener('message', (event) => {
    try
    {
        console.log("Message from server:", event.data);
        const data = event.data;
        
        if (data.compare("-1"))
        {
            emailInput.value = '';
            pwdInput.value = '';
            console.log("Wrong E-mail and/or Password");
        } 
        else
        {
            localStorage.setItem("UserRole", data); //creates local application variable to track UserRole
            window.location.replace("main.html"); //replaces window to user after confirming login
        }
    } catch (error){
        console.error("ERROR parsing message from server:", error);
    }
});

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
