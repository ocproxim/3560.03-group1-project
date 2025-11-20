//conect to websocket server
const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener('open', () => {
    console.log("Connected to database server");
    socket.send(JSON.stringify({fetchSport: "y"}));
});

socket.addEventListener('message', (event) => {
    try {
        //get data from server
        const data = JSON.stringify(event.data);
        console.log(data);
        //check if array
        if (Array.isArray(data) && data.length > 0) {
            if (data[0].hasOwnProperty('sportName')) {
                const sportNames = data.map(item => item.sportName);
                console.log("Fetched sports:", sportNames);
                populateSportsDropdown(sportNames);
            }
        }
    } catch (error) {
        console.error("Error parsing message from server:", error);
    }
});

//startup function
document.addEventListener('DOMContentLoaded', async () => {
    //fetch sport
    
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
    const searchSportForm = document.getElementById('searchSport');
    const searchSeasonForm = document.getElementById('searchSeason');
    const searchTypeForm = document.getElementById('searchType');

    //build JSON
    const message = {
        query: inputString,
        sport: searchSportForm.value,
        season: searchSeasonForm.value,
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
    searchSeasonForm.value = '';
    searchTypeForm.value = '';
});





//populate sports dropdown
function populateSportsDropdown(sportNames) {
    const selectElement = document.getElementById('searchSport');

    //clear existing options
    selectElement.innerHTML = '';
    //create placeholder option

    const defaultOption = document.createElement('option');
    defaultOption.textContent = '-- Please select a sport --';
    defaultOption.value = '';

    //populate options
    sportNames.forEach(sport => {
        const option = document.createElement('option');
        option.value = sport;
        option.textContent = sport;
        selectElement.appendChild(option);
    });

}