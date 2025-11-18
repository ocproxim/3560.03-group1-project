//Link to a Game, Player, StatType via foreign keys
public class StatInstance
{
    public required int statInstanceID;
    public required String timestamp;
    public required float value;

    public static StatInstance FromValues(int id, String timestamp, float value)
    {
        StatInstance statInstance = new()
        {
            statInstanceID = id,
            timestamp = timestamp,
            value = value
        };
        return statInstance;
    }
}


