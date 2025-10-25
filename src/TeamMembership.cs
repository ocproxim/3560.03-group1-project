//Use foreign key to link to a Player and Team for a season

public class TeamMembership
{
    public required String membershipID;
    public required String Season;
    public required int jerseyNumber;

    // Get methods
    public String getSeason() { return Season; }
    public int getJerseyNumber() { return jerseyNumber; }

    // Set methods
    public void setSeason(String newSeason) { Season = newSeason; }
    public void setJerseyNumber(int newJerseyNumber) { jerseyNumber = newJerseyNumber; }
}
