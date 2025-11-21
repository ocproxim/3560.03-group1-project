using Microsoft.Data.Sqlite;

public class Sport
{
    public int sportID;
    public String sportName;

    public Sport(int id, string name)
    {
        sportID = id;
        sportName = name;
    }
    public static Sport FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var name = reader.GetString(1);

        return new Sport(id, name);

    }
    public int getID()
    {
        return sportID;
    }
}


