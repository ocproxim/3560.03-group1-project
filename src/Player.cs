using Microsoft.Data.Sqlite;

public class Player
{
    public int playerID;
    public String name;
    public String dateOfBirth;
    public int height;
    public int weight;

    public Player(int id, string name, string dob, int h, int w)
    {
        playerID = id;
        this.name = name;
        dateOfBirth = dob;
        height = h;
        weight = w;
    }

    // Get methods
    public String getPlayerName() { return name; }
    public DateTime getPlayerDOB() { return DateTime.Parse(dateOfBirth); }
    public int getPlayerHeight() { return height; }
    public int getPlayerWeight() { return weight; }

    // Set methods
    public void setPlayerName(String newName) { name = newName; }
    public void setPlayerHeight(DateTime newDOB) { dateOfBirth = newDOB.ToShortDateString(); }
    public void setPlayerHeight(int newHeight) { height = newHeight; }
    public void setPlayerWeight(int newWeight) { weight = newWeight; }

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
