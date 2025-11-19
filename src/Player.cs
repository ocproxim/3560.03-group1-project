using Microsoft.Data.Sqlite;

public class Player
{
    public int playerID;
    public String Name;
    public String dateOfBirth;
    public int Height;
    public int Weight;

    public Player(int id, string name, string dob, int h, int w)
    {
        playerID = id;
        Name = name;
        dateOfBirth = dob;
        Height = h;
        Weight = w;
    }

    // Get methods
    public String getPlayerName() { return Name; }
    public DateTime getPlayerDOB() { return DateTime.Parse(dateOfBirth); }
    public int getPlayerHeight() { return Height; }
    public int getPlayerWeight() { return Weight; }

    // Set methods
    public void setPlayerName(String newName) { Name = newName; }
    public void setPlayerHeight(DateTime newDOB) { dateOfBirth = newDOB.ToString(); }
    public void setPlayerHeight(int newHeight) { Height = newHeight; }
    public void setPlayerWeight(int newWeight) { Weight = newWeight; }

    //DB methods
    public static Player FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var name = reader.GetString(1);
        var dob = reader.GetString(2);
        var height = reader.GetInt32((3));
        var weight = reader.GetInt32((4));

        return new Player(id, name, dob, height, weight);
    }
}
