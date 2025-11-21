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
        else if( isValidStatData(data) ) {
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
});

// Checks if an object is valid data
function isValidStatData(data) {
    if (typeof data !== "object" || data === null) return false;

    // If data is a single record (player/team/game)
    const values = Object.values(data);
    if (values.length > 0 && values.every(arr => Array.isArray(arr) && arr.every(n => typeof n === 'number'))) {
        return true;
    }

    // If data is multiple records keyed by name/id
    return Object.values(data).every(record => {
        if (typeof record !== 'object' || record === null) return false;
        const stats = Object.values(record);
        return stats.length > 0 && stats.every(arr => Array.isArray(arr) && arr.every(n => typeof n === 'number'));
    });
}


//create stats table
function createStatsTable(statsData) {
    console.log("Creating table for stats:", statsData);

    const tableHead = document.getElementById('tableHead');
    const tableBody = document.getElementById('tableBody');
    const container = document.querySelector('.search-results');

    // Clear previous table
    tableHead.innerHTML = '';
    tableBody.innerHTML = '';

    const players = Object.keys(statsData);
    if (players.length === 0) return;

    // Use first player's stats as columns
    const statColumns = Object.keys(statsData[players[0]]);

    // Build table header
    const headerRow = document.createElement('tr');

    // Player column
    const playerTh = document.createElement('th');
    playerTh.textContent = 'Player';
    headerRow.appendChild(playerTh);

    // Stat columns
    statColumns.forEach(stat => {
        const th = document.createElement('th');
        th.textContent = stat;
        headerRow.appendChild(th);
    });

    // Actions column
    const actionTh = document.createElement('th');
    actionTh.textContent = 'Actions';
    headerRow.appendChild(actionTh);

    tableHead.appendChild(headerRow);

    // Build table body
    players.forEach(playerName => {
        const playerStats = statsData[playerName];

        const averages = {};
        for (const stat in playerStats) {
            const arr = playerStats[stat];
            const avg = arr.reduce((sum, val) => sum + val, 0) / arr.length;
            averages[stat] = Math.floor(avg);
        }

        const row = document.createElement('tr');

        // Player name
        const nameTd = document.createElement('td');
        nameTd.textContent = playerName;
        row.appendChild(nameTd);

        // Stat columns
        statColumns.forEach(stat => {
            const td = document.createElement('td');
            td.textContent = averages[stat] !== undefined ? averages[stat] : '';
            row.appendChild(td);
        });

        // Actions
        const actionTd = document.createElement('td');

        const editBtn = document.createElement('button');
        editBtn.textContent = 'Edit';
        editBtn.classList.add('edit-btn');
        editBtn.dataset.player = playerName;
        editBtn.addEventListener('click', () => editRecord(playerName));

        const deleteBtn = document.createElement('button');
        deleteBtn.textContent = 'Delete';
        deleteBtn.classList.add('delete-btn');
        deleteBtn.dataset.player = playerName;
        deleteBtn.addEventListener('click', () => deleteRecord(playerName));

        actionTd.appendChild(editBtn);
        actionTd.appendChild(deleteBtn);
        row.appendChild(actionTd);

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

//delete record
function deleteRecord(playerName) {
    if (!confirm(`Delete all stats for ${playerName}?`)) return;

    const msg = {
        action: "deleteRecord",
        player: playerName
    };

    socket.send(JSON.stringify(msg));
    console.log("Sent delete request:", msg);
}

//edit record
function editRecord(playerName) {
    const newName = prompt("Enter new name for this player:", playerName);
    if (!newName || newName.trim() === "") return;

    const msg = {
        action: "updateRecord",
        oldName: playerName,
        newName: newName.trim()
    };

    socket.send(JSON.stringify(msg));
    console.log("Sent update request:", msg);
}
