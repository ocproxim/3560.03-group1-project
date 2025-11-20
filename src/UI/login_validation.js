//get string input
const loginForm = document.getElementById('login');
const emailInput = document.getElementById('emailInput');
const pwdInput = document.getElementById('pwdInput');

loginForm.addEventListener('submit', (e) => {
    // stop page reload
    // e.preventDefault(); 

    // get the login input
    const emailString = emailInput.value;
    const pwdString = pwdInput.value;

    //build JSON
    const message = {
        email: emailString,
        password: pwdString
    };

    //send to websocket server
    if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify(message));
        console.log("Sent:", message);
    }
});
