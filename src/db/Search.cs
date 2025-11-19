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
        if (db == null)
        {
            Console.WriteLine("DB connection failed");
            return null;
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

        var results = new Dictionary<String, List<PlayerGameStats>>();

        switch (type)
        {
            case SearchType.Game:
                break;
            case SearchType.Team:
                var teams = db.GetTeams();
                var potentialTeams = from team in teams where Fuzz.Ratio($"{team.homeTown} {team.teamName}", query) > 70 select team;
                if (potentialTeams.Count() == 0)
                {
                    Console.WriteLine("Fuzzy team search found nothing");
                }
                foreach (var team in teams)
                {
                    var teamPlayers = db.GetPlayersByTeam(team);
                    foreach (var player in teamPlayers)
                    {
                        var stats = GetStatsForPlayer(db, player);
                        if (stats.Count != 0)
                        {
                            var key = $"{player.Name} (ID {player.playerID})";
                            results.Add(key, stats);
                        }
                    }
                }
                break;
            case SearchType.Player:
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

                foreach (var player in potentialPlayers)
                {
                    var stats = GetStatsForPlayer(db, player);
                    if (stats.Count != 0)
                    {
                        var key = $"{player.Name} (ID {player.playerID})";
                        results.Add(key, stats);
                    }
                }
                break;
            default:
                Console.WriteLine("Invalid query type");
                return null;

        }
        var j = JsonSerializer.Serialize(results);
        return j;

    }

    static List<PlayerGameStats> GetStatsForPlayer(StatsDB db, Player player)
    {
        var sportStatKinds = db.GetStatKinds();
        List<StatData> statData = new List<StatData>();
        var games = db.GetGamesByPlayer(player);
        if (games.Count == 0)
        {
            Console.WriteLine("Games list was empty");
        }

        var retVal = new List<PlayerGameStats>();

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


                statData.Add(new StatData
                {
                    statName = statKind.statName,
                    statValue = stat.value
                });

            }
            if (statData.Count == 0) { continue; }

            var g = new PlayerGameStats
            {
                gameTime = game.gameTime,
                venue = game.venue,
                homeScore = game.homeScore,
                awayScore = game.awayScore,
                stats = statData
            };

            retVal.Add(g);
        }
        return retVal;
    }
}



//A player's stats for a single game
public class PlayerGameStats
{
    public required String gameTime { get; set; }
    public required String venue { get; set; }

    public required float homeScore { get; set; }

    public required float awayScore { get; set; }

    public required List<StatData> stats { get; set; }

}
//An individual instance of a stat in a game
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
