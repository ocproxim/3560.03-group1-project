//Link teams in this game via foreign key for home and away team

public class Game
{
    public required String gameID;
    public required float homeScore;
    public required float awayScore;
    public required DateTime gameTime;
    public required String venue;

    // Get methods
    public float getGameHomeScore() { return homeScore; }
    public float getGameAwayScore() { return awayScore; }
    public DateTime getGameTime() { return gameTime; }
    public String getGameVenue() { return venue; }

    // Set methods
    public void setGameHomeScore(float newHomeScore) { homeScore = newHomeScore; }
    public void setGameAwayScore(float newAwayScore) { awayScore = newAwayScore; }
    public void setGameTime(DateTime newGameTime) { gameTime = newGameTime; }
    public void setGameVenue(String newVenue) { venue = newVenue; }
}
