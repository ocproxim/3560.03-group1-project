//Use foreign key to link to a Sport

using Microsoft.Data.Sqlite;

public class Team
{
    public int teamID;
    public String teamName;
    public String homeTown;

    public Team(int id, string name, string town)
    {
        teamID = id;
        teamName = name;
        homeTown = town;
    }

    // Get methods
    public String getTeamName() { return teamName; }
    public String getTeamHome() { return homeTown; }

    // Set methods
    public void setTeamName(String newName) { teamName = newName; }
    public void setTeamHome(String newHomeTown) { homeTown = newHomeTown; }

    public static Team FromReader(SqliteDataReader reader)
    {
        var id = reader.GetInt32(0);
        var name = reader.GetString(1);
        var town = reader.GetString(2);

        return new Team(id, name, town);

    }
}
