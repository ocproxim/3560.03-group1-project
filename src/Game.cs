//Link teams in this game via foreign key for home and away team

public class Game
{
    public required String gameID;
    public required float homeScore;
    public required float awayScore;
    public required DateTime gameTime;
    public required String venue;
}
