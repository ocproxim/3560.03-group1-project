//Link teams in this game via foreign key for home and away team

using Microsoft.Data.Sqlite;

public class Game
{
    public int gameID;
    public int homeTeam;
    public int awayTeam;
    public float homeScore;
    public float awayScore;
    public String gameTime;
    public String venue;

    public Game(int id, int hID, int aID, float hscore, float ascore, string time, string v)
    {
        gameID = id;
        homeTeam = hID;
        awayTeam = aID;
        homeScore = hscore;
        awayScore = ascore;
        gameTime = time;
        venue = v;
    }


    // Get methods
    public float getGameHomeScore() { return homeScore; }
    public float getGameAwayScore() { return awayScore; }
    public DateTime getGameTime() { return DateTime.Parse(gameTime); }
    public String getGameVenue() { return venue; }

    // Set methods
    public void setGameHomeScore(float newHomeScore) { homeScore = newHomeScore; }
    public void setGameAwayScore(float newAwayScore) { awayScore = newAwayScore; }
    public void setGameTime(DateTime newGameTime) { gameTime = newGameTime.ToString(); }
    public void setGameVenue(String newVenue) { venue = newVenue; }

    //DB methods
    public static Game FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);

        var hID = reader.GetInt32(1);

        var aID = reader.GetInt32(2);
        var hscore = reader.GetFloat(3);
        var ascore = reader.GetFloat(4);
        var time = reader.GetString(5);
        var v = reader.GetString(6);

        return new Game(id, hID, aID, hscore, ascore, time, v);
    }

}
