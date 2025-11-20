/*
 *query JSON structure{
  "query": "a",
  "sport": "baseball | basketball | sport",
  "type": "game | team | player"
}
 * */

using Microsoft.Data.Sqlite;
using System.Text.Json;
using FuzzySharp;



namespace Search
{
    public enum SearchType
    {
        Game,
        Team,
        Player
    }

    public class DBConnection
    {
        SqliteConnection connection;

        public DBConnection(String databasePath)
        {
            connection = new SqliteConnection($"Data Source={databasePath}; Mode = ReadWrite;");
            if (connection.State != System.Data.ConnectionState.Open)
            {
                connection.Open();
            }
        }

        public void BasicWebQuery(string sportName, SearchType type, String query)
        {
            if (connection == null)
            {
                return;
            }
            switch (type)
            {
                case SearchType.Game:
                    break;
                case SearchType.Team:
                    break;
                case SearchType.Player:
                    var player_list = connection.CreateCommand();
                    player_list.CommandText = "SELECT playerID,name FROM Players";
                    var playerReader = player_list.ExecuteReader();
                    List<int> playerIDs = new List<int>();
                    List<string> playerNames = new List<string>();
                    while (playerReader.Read())
                    {
                        //Use a fuzzy string matching algorithm to compare query to player names, add them to results if they match above a certain threshold
                        var playerID = playerReader.GetInt32(0);
                        var name = playerReader.GetString(1);
                        var ratio = Fuzz.Ratio(query, name);
                        if (ratio > 70)
                        {
                            playerIDs.Add(playerID);
                            playerNames.Add(name);
                        }
                    }
                    //Get sportID from sportName
                    var sports = connection.CreateCommand();
                    sports.CommandText = "SELECT sportID FROM Sports WHERE UPPER(sportName) like UPPER($sportName)";
                    sports.Parameters.AddWithValue("$sportName", sportName);
                    var sportReader = sports.ExecuteReader();
                    int sportID = -1;
                    if (sportReader.Read())
                    {

                        sportID = sportReader.GetInt32(0);
                    }

                    var gameList = connection.CreateCommand();
                    gameList.CommandText = "SELECT gameID,homeScore,awayScore,gameTime,venue FROM Games";
                    var gameReader = gameList.ExecuteReader();
                    List<PlayerStatResults> results = new List<PlayerStatResults>();
                    while (gameReader.Read())
                    {
                        var gameID = gameReader.GetInt32(0);

                        var homeScore = gameReader.GetFloat(1);

                        var awayScore = gameReader.GetFloat(2);

                        var gameTime = gameReader.GetString(3);

                        var venue = gameReader.GetString(4);

                        for (int i = 0; i < playerIDs.Count; i++)
                        {
                            //Get stats for this player in this game
                            var statList = connection.CreateCommand();
                            statList.CommandText = "SELECT statKindID,value FROM StatInstances WHERE gameID = $gameID AND playerID = $playerID";
                            statList.Parameters.AddWithValue("$gameID", gameID);
                            statList.Parameters.AddWithValue("$playerID", playerIDs[i]);
                            var statReader = statList.ExecuteReader();
                            List<StatData> stats = new List<StatData>();
                            while (statReader.Read())
                            {
                                var statKindID = statReader.GetInt32(0);
                                var statValue = statReader.GetFloat(1);
                                var statNameCmd = connection.CreateCommand();
                                statNameCmd.CommandText = "SELECT statName FROM StatKinds WHERE statKindID = $statKindID AND sportID = $sportID";
                                statNameCmd.Parameters.AddWithValue("$statKindID", statKindID);
                                statNameCmd.Parameters.AddWithValue("$sportID", sportID);
                                var statNameReader = statNameCmd.ExecuteReader();

                                String statName = "";
                                if (statNameReader.Read())
                                {
                                    statName = statNameReader.GetString(0);
                                }

                                stats.Add(new StatData
                                {
                                    statName = statName,
                                    statValue = statValue
                                });
                            }
                            //Only add to results if there are stats for this player in this game
                            if (stats.Count > 0)
                            {
                                //Find existing player in results
                                var playerStatResult = results.Find(p => p.name == playerNames[i]);
                                if (playerStatResult == null)
                                {
                                    playerStatResult = new PlayerStatResults
                                    {
                                        name = playerNames[i],
                                        gameStats = new List<PlayerGameStats>()
                                    };
                                    results.Add(playerStatResult);
                                }
                                playerStatResult.gameStats.Add(new PlayerGameStats
                                {
                                    gameTime = gameTime,
                                    venue = venue,
                                    homeScore = homeScore,
                                    awayScore = awayScore,
                                    stats = stats
                                });
                            }
                        }
                    }
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

        public string FetchSports()
        {
            var sportNames = new List<object>();
            string query = "SELECT sportID, sportName FROM Sports ORDER BY sportName"; 

            using (var command = new SqliteCommand(query, connection))
            {
            using (var reader = command.ExecuteReader())
            {
                while (reader.Read())
                {
                        sportNames.Add(new { sportName = reader.GetString(1) });
                }
            }
        }
            string json = JsonSerializer.Serialize(sportNames);
            return json;
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




