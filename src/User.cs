class User
{
    public required String userID;
    public required String email;
    public required ulong passwordHash;
    public required UserRole role;

}

enum UserRole
{
    User, Admin, Scorekeeper
}
