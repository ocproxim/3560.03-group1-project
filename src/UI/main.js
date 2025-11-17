const searchForm = document.getElementById('search');
const searchInput = document.getElementById('searchInput');
const outputDisplay = document.getElementById('output');

searchForm.addEventListener('submit', function(event) {
    // stop page reload
    event.preventDefault(); 

    // get the search input
    const inputString = searchInput.value;

    // print to console
    console.log('User entered string:', inputString);

    // clear the input field
    searchInput.value = '';
});