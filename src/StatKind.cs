//Link to Sport via foreign key 

using Microsoft.Data.Sqlite;

public class StatKind
{
    public int statKindID;
    public int sportID;
    public String statName;
    public String unit;

    public StatKind(int id, int spId, string name, string u)
    {
        statKindID = id;
        sportID = spId;
        statName = name;
        unit = u;
    }
    public static StatKind FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var spId = reader.GetInt32(1);
        var name = reader.GetString(2);
        var u = reader.GetString(3);

        return new StatKind(id, spId, name, u);

    }
}
