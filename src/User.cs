class User
{
    public required int userID;
    public required String email;
    public required String passwordHash;
    public required UserRole role;

    // Get methods
    public String getUserEmail() { return email; }
    public String getUserPasswordHash() { return passwordHash; }
    public UserRole getUserRole() { return role; }

    // Set methods
    public void setUserEmail(String newEmail) { email = newEmail; }
    public void setUserPasswordHash(String newPassword) { passwordHash = newPassword; }
    public void setUserRole(UserRole newRole) { role = newRole; }

}

enum UserRole
{
    User = 0, Admin = 1, Scorekeeper = 2
}


