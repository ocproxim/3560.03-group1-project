class User
{
    public required String userID;
    public required String email;
    public required String passwordHash;
    public required UserRole role;

    // Get methods
    public String getUserEmail() { return email; }
    public String getUserPasswordHash() { return passwordHash; }
    public UserRole getUserRole() { return role; }

    // Set methods
    public void setUserEmail(String newEmail) { email = newEmail; }
    public void setUserPasswordHash(ulong newPassword) { passwordHash = newPassword; }
    public void setUserRole(UserRole newRole) { role = newRole; }

}

enum UserRole
{
    User, Admin, Scorekeeper
}


