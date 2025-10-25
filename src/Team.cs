//Use foreign key to link to a Sport

public class Team
{
    public required String teamID;
    public required String teamName;
    public required String homeTown;

    // Get methods
    public String getTeamName() { return teamName; }
    public String getTeamHome() { return homeTown; }

    // Set methods
    public void setTeamName(String newName) { teamName = newName; }
    public void setTeamHome(String newHomeTown) { homeTown = newHomeTown; }
}
