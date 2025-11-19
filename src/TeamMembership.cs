//Use foreign key to link to a Player and Team for a season

using Microsoft.Data.Sqlite;

public class TeamMembership
{
    public int membershipID;
    public int playerID;
    public int teamID;
    public String season;
    public int jerseyNumber;

    public TeamMembership(int id, int pID, int tID, string s, int j)
    {
        membershipID = id;
        playerID = pID;
        teamID = tID;

        season = s;
        jerseyNumber = j;
    }

    // Get methods
    public String getSeason() { return season; }
    public int getJerseyNumber() { return jerseyNumber; }

    // Set methods
    public void setSeason(String newSeason) { season = newSeason; }
    public void setJerseyNumber(int newJerseyNumber) { jerseyNumber = newJerseyNumber; }

    public static TeamMembership FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var pID = reader.GetInt32(1);
        var tID = reader.GetInt32(2);
        var s = reader.GetString(3);
        var j = reader.GetInt32(4);

        return new TeamMembership(id, pID, tID, s, j);

    }

}
