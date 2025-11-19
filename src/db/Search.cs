/*
 *query JSON structure{
  "query": "a",
  "sport": "baseball | basketball | sport",
  "type": "game | team | player"
}
 * */

using System.Text.Json;
using FuzzySharp;



public class Search
{
    public enum SearchType
    {
        Game,
        Team,
        Player
    }
    public static string? BasicWebQuery(StatsDB db, string searchSport, SearchType type, String query)
    {
        Console.WriteLine(type);
        if (db == null)
        {
            Console.WriteLine("DB connection failed");
            return null;
        }
        switch (type)
        {
            case SearchType.Game:
                Console.WriteLine("Game");
                break;
            case SearchType.Team:

                Console.WriteLine("Team");
                break;
            case SearchType.Player:
                Console.WriteLine("Player");
                var players = db.GetPlayers();
                if (players.Count == 0)
                {
                    Console.WriteLine("Players list was empty");
                }

                //Use a fuzzy string matching algorithm to compare query to player names, add them to results if they match above a certain threshold
                var potentialPlayers = from player in players
                                       where Fuzz.Ratio(player.Name, query) > 70
                                       select player;
                if (potentialPlayers.Count() == 0)
                {
                    Console.WriteLine("Fuzzy player search found nothing");
                }

                //Get sportID from sportName
                var sports = db.GetSports();
                var thisSport = sports.Find(sport => sport.sportName.ToLower() == searchSport.ToLower());
                if (thisSport == null)
                {

                    Console.WriteLine("Did not find a sport that matched query");
                    return null;
                }

                var sportStatKinds = db.GetStatKindsForSport(thisSport);
                if (sportStatKinds.Count == 0)
                {
                    Console.WriteLine("Sport stat kinds was empty");
                }

                var results = new List<PlayerStatResults>();
                foreach (var player in potentialPlayers)
                {
                    List<StatData> playerGameStats = new List<StatData>();
                    var games = db.GetGamesByPlayer(player);
                    if (games.Count == 0)
                    {
                        Console.WriteLine("Games list was empty");
                    }
                    foreach (var game in games)
                    {
                        var gameStats = db.GetStatInstancesForGame(game);
                        if (gameStats.Count == 0)
                        {
                            Console.WriteLine("Stats by game query was empty");
                        }
                        var statsIter = from stat in gameStats
                                        where stat.playerID == player.playerID && stat.gameID == game.gameID
                                        select stat;

                        foreach (var stat in statsIter)
                        {
                            var statKind = sportStatKinds.Find(kind => kind.statkindID == stat.statKindID);
                            if (statKind == null)
                            {

                                Console.WriteLine("Stat kind was empty");
                                continue;
                            }
                            if (statKind.statName == null)
                            {
                                Console.WriteLine("Stat name was empty");
                                continue;

                            }


                            playerGameStats.Add(new StatData
                            {
                                statName = statKind.statName,
                                statValue = stat.value
                            });

                        }

                        //Only add to results if there are stats for this player in this game
                        if (playerGameStats.Count > 0)
                        {
                            //Find existing player in results
                            var playerStatResult = results.Find(p => p.name == player.Name);
                            if (playerStatResult == null)
                            {

                                playerStatResult = new PlayerStatResults
                                {
                                    name = player.Name,
                                    gameStats = new List<PlayerGameStats>()
                                };
                                results.Add(playerStatResult);
                            }

                            playerStatResult.gameStats.Add(new PlayerGameStats
                            {
                                gameTime = game.gameTime,
                                venue = game.venue,
                                homeScore = game.homeScore,
                                awayScore = game.awayScore,
                                stats = playerGameStats
                            });
                        }

                    }
                }
                var j = JsonSerializer.Serialize(results);
                return j;
            default:
                Console.WriteLine("Invalid query type");
                return null;

        }
        Console.WriteLine("Unimplemented");
        return null;
    }

}

//Structure for returning data to the frontend
public class PlayerStatResults
{
    public required String name { get; set; }

    public required List<PlayerGameStats> gameStats { get; set; }

}

public class PlayerGameStats
{
    public required String gameTime { get; set; }
    public required String venue { get; set; }

    public required float homeScore { get; set; }

    public required float awayScore { get; set; }

    public required List<StatData> stats { get; set; }

}
public class StatData
{
    public required String statName { get; set; }
    public required float statValue { get; set; }


}



/*
                 Console.WriteLine($"{results.Count}");
                foreach (var result in results)
                {
                    Console.WriteLine($"Player Name: {result.name}");
                    foreach (var gameStat in result.gameStats)
                    {

                        Console.WriteLine($"\tGame Time: {gameStat.gameTime}, Venue: {gameStat.venue}, Home Score: {gameStat.homeScore}, Away Score: {gameStat.awayScore}");
                        foreach (var stat in gameStat.stats)
                        {
                            Console.WriteLine($"\t\tStat Name: {stat.statName}, Stat Value: {stat.statValue}");
                        }
                    }

                }


 * */
