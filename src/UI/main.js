//conect to websocket server
const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener('open', () => {
    console.log("Connected to database server");
});

socket.addEventListener('message', (event) => {
    console.log("Message from server:", event.data);
});


//get string input
const searchForm = document.getElementById('search');
const searchInput = document.getElementById('searchInput');

searchForm.addEventListener('submit', function(event) {
    // stop page reload
    event.preventDefault(); 

    // get the search input
    const inputString = searchInput.value;
    //get dropdown input
    const searchSportForm = document.getElementById('sportType');
    const searchTypeForm = document.getElementById('searchType');

    //build JSON
    const message = {
        query: inputString,
        sport: searchSportForm.value,
        type: searchTypeForm.value
    };

    //send to websocket server
    if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify(message));
        console.log("Sent:", message);
    }

    // clear the input field
    searchInput.value = '';
    searchSportForm.value = '';
    searchTypeForm.value = '';
});