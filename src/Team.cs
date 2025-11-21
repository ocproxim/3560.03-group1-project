//Use foreign key to link to a Sport

using Microsoft.Data.Sqlite;

public class Team
{
    public int teamID;
    public int sportID;
    public String teamName;
    public String homeTown;

    public Team(int id, int sID, string name, string town)
    {
        teamID = id;
        sportID = sID;
        teamName = name;
        homeTown = town;
    }

    // Get methods
    public String getTeamName() { return teamName; }
    public String getTeamHome() { return homeTown; }
    public int getTeamID() { return teamID; }


    // Set methods
    public void setTeamName(String newName) { teamName = newName; }
    public void setTeamHome(String newHomeTown) { homeTown = newHomeTown; }

    public static Team FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var sID = reader.GetInt32(1);
        var name = reader.GetString(2);
        var town = reader.GetString(3);

        return new Team(id, sID, name, town);

    }
}
