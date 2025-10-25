class User
{
    public required String userID;
    public required String email;
    public required ulong passwordHash;
    public required UserRole role;

    // Get methods
    public String getUserEmail() { return email; }
    public ulong getUserPasswordHash() { return passwordHash; }
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


