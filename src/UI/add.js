//conect to websocket server
const socket = new WebSocket("ws://localhost:8080");

socket.addEventListener('open', () => {
    console.log("Connected to database server");
    hideElements();
});

// Grab the type select and WebSocket
const typeSelect = document.getElementById('addType');

// Add submit listener to the whole page (or a wrapper form)
document.addEventListener('submit', function (event) {
    event.preventDefault(); // stop page reload

    const selectedValue = typeSelect.value;
    if (!selectedValue) return console.error("No type selected");

    // Grab the currently visible form
    const visibleForm = document.getElementById(`${selectedValue}Form`);
    if (!visibleForm || visibleForm.style.display === "none") {
        return console.error("Visible form not found for type:", selectedValue);
    }

    // Build message object dynamically from the form inputs
    const inputs = visibleForm.querySelectorAll('input, select, textarea');
    const message = { add: selectedValue };

    inputs.forEach(input => {
        if (input.type === "submit") return; // skip submit buttons
        message[input.name] = input.value;
    });

    console.log("Submitting message:", message);

    // Send via WebSocket
    if (socket && socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify(message));
        console.log("Message sent!");
    } else {
        console.error("WebSocket is not connected");
    }

    // Clear inputs after sending
    inputs.forEach(input => {
        if (input.type !== "submit") input.value = '';
    });

    // Optionally hide the form again
    visibleForm.style.display = "none";
});


//find out what type user selected
typeSelect.addEventListener('change', () => {
    const selectedValue = typeSelect.value;
    
    hideElements();

    switch (selectedValue) {
        case 'player':
            document.getElementById("playerForm").style.display = "block";
            break;
        case 'team':
            document.getElementById("teamForm").style.display = "block";
            break;
        case 'sport':
            document.getElementById("sportForm").style.display = "block";
            break;
        case 'game':
            document.getElementById("gameForm").style.display = "block";
            break;
        default:
            // nothing to show
            break;
    }
});

// Hides the elements on this page so they don't overlap
function hideElements() {
    document.getElementById("playerForm").style.display = "none";
    document.getElementById("teamForm").style.display = "none";
    document.getElementById("sportForm").style.display = "none";
    document.getElementById("gameForm").style.display = "none";
}
