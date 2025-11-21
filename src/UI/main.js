//conect to websocket server
const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener('open', () => {
    console.log("Connected to database server");
    socket.send(JSON.stringify({fetchSport: "y"}));
});

socket.addEventListener('message', (event) => {
    try {
        //get data from server
        //console.log("Message from server:", event.data);
        const data = JSON.parse(event.data);
        console.log("Parsed data:", data);
        
        //check if array, check if retrieved sports
        if (Array.isArray(data) && data.length > 0 && data[0].hasOwnProperty('sportName')) {
            const sportNames = data
            .map(sport => ({sportName: sport.sportName}));
            console.log("Fetched sports:", sportNames);
            populateSportsDropdown(sportNames);
        }
        //check if player stats
        else if( isPlayerGameData(data) ) {
            //create table for stats
            createStatsTable(data);
        }
    } catch (error) {
        console.error("Error parsing message from server:", error);
    }
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

// Checks if an object is player game data
function isPlayerGameData(data) {
    if (typeof data !== "object" || data === null) return false;

    // Check if it's a single player's stats object
    const values = Object.values(data);
    if (values.length === 0) return false;

    // If all values are arrays of numbers of the same length, it's a player object
    if (values.every(
        arr => Array.isArray(arr) && arr.every(n => typeof n === "number")
    )) {
        const len = values[0].length;
        if (values.every(arr => arr.length === len)) return true;
        else return false;
    }

    // If it's an object with player names as keys, check if at least one value is a player object
    const keys = Object.keys(data);
    return keys.some(key => {
        const playerObj = data[key];
        if (typeof playerObj !== "object" || playerObj === null) return false;
        const stats = Object.values(playerObj);
        if (stats.length === 0) return false;
        return stats.every(arr => Array.isArray(arr) && arr.every(n => typeof n === "number")) &&
               stats.every(arr => arr.length === stats[0].length);
    });
}



//create stats table
function createStatsTable(statsData) {
    console.log("Creating table for stats:", statsData);
    const tableHead = document.getElementById('tableHead');
    const tableBody = document.getElementById('tableBody');
    const container = document.querySelector('.search-results');

    tableHead.innerHTML = '';
    tableBody.innerHTML = '';

    const players = Object.keys(statsData);
    if (players.length === 0) return;

    // find first player that is valid game data
    const firstPlayer = players.find(name => isPlayerGameData(statsData[name]));
    if (!firstPlayer) {
        console.warn("No valid player game data found.");
        return;
    }

    const statColumns = Object.keys(statsData[firstPlayer]);

    // build header
    const headerRow = document.createElement("tr");
    headerRow.innerHTML =
        `<th>Player</th>` +
        statColumns.map(stat => `<th>${stat}</th>`).join("");
    tableHead.appendChild(headerRow);

    // build each row
    players.forEach(playerName => {
        const playerStats = statsData[playerName];
        if (!isPlayerGameData(playerStats)) return;

        const averages = averageAndFloorStats(playerStats);

        const row = document.createElement("tr");
        row.innerHTML =
            `<td>${playerName}</td>` +
            statColumns.map(stat => `<td>${averages[stat]}</td>`).join("");

        tableBody.appendChild(row);
    });
    container.style.display = 'block';
}



//populate sports dropdown
function populateSportsDropdown(sportNames) {
    const selectElement = document.getElementById('searchSport');

    //clear existing options
    selectElement.innerHTML = '';
    //create placeholder option

    const defaultOption = document.createElement('option');
    defaultOption.textContent = '-- Select a sport --';
    defaultOption.value = '';
    selectElement.appendChild(defaultOption);

    //populate options
    sportNames.forEach(sport => {
        const option = document.createElement('option');
        option.value = sport.sportName;
        option.textContent = sport.sportName;
        selectElement.appendChild(option);
    });
}

//average player stats and floor
    function averageAndFloorStats(playerStats) {
        const averagedStats = {};

        for (const stat in playerStats) {
            const values = playerStats[stat];
            const avg = values.reduce((sum, val) => sum + val, 0) / values.length;
            averagedStats[stat] = Math.floor(avg);
        }
        return averagedStats;
    }