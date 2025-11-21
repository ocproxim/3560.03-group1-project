/*
 *query JSON structure{
  "query": "a",
  "sport": "baseball | basketball | sport",
  "type": "game | team | player"
}
 * */

using System.Text.Json;
using FuzzySharp;
using Microsoft.Data.Sqlite;



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

        //A Dictionary mapping a player to another dictionary where the key is the stat name and the value is a list of all the instances that were found during the search
        var results = new Dictionary<String, Dictionary<String, List<float>>>();

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
                        var games = db.GetGamesByPlayer(player);
                        var stats = GetStatsForPlayerInGames(db, player, games);
                        if (stats.Count != 0)
                        {
                            var key = $"{player.name} (ID {player.playerID})";
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
                                       where Fuzz.Ratio(player.name, query) > 70
                                       select player;
                if (potentialPlayers.Count() == 0)
                {
                    Console.WriteLine("Fuzzy player search found nothing");
                }

                foreach (var player in potentialPlayers)
                {
                    var games = db.GetGamesByPlayer(player);
                    var stats = GetStatsForPlayerInGames(db, player, games);
                    if (stats.Count != 0)
                    {
                        var key = $"{player.name} (ID {player.playerID})";
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

    static Dictionary<String, List<float>> GetStatsForPlayerInGames(StatsDB db, Player player, IEnumerable<Game> games)
    {
        var sportStatKinds = db.GetStatKinds();
        Dictionary<String, List<float>> statData = new Dictionary<String, List<float>>();


        var retVal = new Dictionary<String, List<float>>();

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

                var statKind = sportStatKinds.Find(kind => kind.statKindID == stat.statKindID);
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



                if (retVal.ContainsKey(statKind.statName))
                {

                    var statList = retVal.GetValueOrDefault(statKind.statName);
                    statList.Add(stat.value);

                }
                else
                {
                    retVal.Add(statKind.statName, new List<float>());
                    var statList = retVal.GetValueOrDefault(statKind.statName);
                    statList.Add(stat.value);

                }

            }

        }

        return retVal;
    }
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

                    break;
                default:
                    break;

            }
            return;
        }

        

        //end class
    }



public class PlayerStatResults
{
    public required String name;

    public required List<PlayerGameStats> gameStats;

}

public class PlayerGameStats
{
    public required String gameTime;
    public required String venue;

    public required float homeScore;

    public required float awayScore;

    public required List<StatData> stats;

}
public class StatData
{
    public required String statName;
    public required float statValue;

}
}





 * */
