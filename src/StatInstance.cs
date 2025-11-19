//Link to a Game, Player, StatType via foreign keys
using Microsoft.Data.Sqlite;

public class StatInstance
{
    public int statInstanceID;
    public int playerID;
    public int gameID;
    public int statKindID;
    public String timestamp;
    public float value;


    public StatInstance(int id, int pId, int gId, int skId, String t, float v)
    {
        statInstanceID = id;
        playerID = pId;
        gameID = gId;
        statKindID = skId;
        timestamp = t;
        value = v;
    }

    public static StatInstance FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);

        var pId = reader.GetInt32(1);

        var gId = reader.GetInt32(2);

        var skId = reader.GetInt32(3);

        var t = reader.GetString(4);

        var v = reader.GetFloat(5);

        return new StatInstance(id, pId, gId, skId, t, v);
    }
}


