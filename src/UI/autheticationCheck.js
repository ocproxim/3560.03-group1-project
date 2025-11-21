/*
    Javascript to check user if they are Admin or scorekeeper before letting them through CRUD functionality
*/
class AuthCheck {
    constructor(){
        document.querySelector("body").style.display = "none"; //hides window from user before checking authentication
        const UserRole = localStorage.getItem("UserRole");
        this.validateAuthentication(UserRole);
    }

    validateAuthentication(UserRole){
        if (UserRole == 0){
            window.location.replace("main.html"); //replaces window when user is not logged in or a normal user
        } else if (UserRole == null){
            window.location.replace("login.html");
        } else {
            document.querySelector("body").style.display = "block"; //shows window to user after confirming authentication
        }
    }

    logOut(){
        localStorage.removeItem("UserRole");
        window.location.replace("main.html"); //replaces window
    }
}

const AuthenticationCheck = new AuthCheck();
/*
document.querySelector(".logout").addEventListener("click", (e) => {
    AuthCheck.logOut();
})
*/