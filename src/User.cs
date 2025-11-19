using Microsoft.Data.Sqlite;

public class User
{
    public int userID;
    public String email;
    public String passwordHash;
    public UserRole role;

    public User(int id, string e, string pHash, UserRole r)
    {
        userID = id;
        email = e;
        passwordHash = pHash;
        role = r;
    }

    // Get methods
    public String getUserEmail() { return email; }
    public String getUserPasswordHash() { return passwordHash; }
    public UserRole getUserRole() { return role; }

    // Set methods
    public void setUserEmail(String newEmail) { email = newEmail; }
    public void setUserPasswordHash(String newPassword) { passwordHash = newPassword; }
    public void setUserRole(UserRole newRole) { role = newRole; }

    public static User FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var e = reader.GetString(1);
        var pHash = reader.GetString(2);
        var r = (UserRole)reader.GetInt32(3);

        return new User(id, e, pHash, r);

    }
}

public enum UserRole
{
    User = 0, Admin = 1, Scorekeeper = 2
}


