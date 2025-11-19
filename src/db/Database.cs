using Microsoft.Data.Sqlite;

public class StatsDB
{
    SqliteConnection connection;

    public StatsDB(String databasePath)
    {
        connection = new SqliteConnection($"Data Source={databasePath}; Mode = ReadWrite;");
        if (connection.State != System.Data.ConnectionState.Open)
        {
            connection.Open();
        }
    }
    public List<Player> GetPlayers()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT playerID,name,dateOfBirth,height,weight FROM Players";
        var reader = sqlCommand.ExecuteReader();

        List<Player> retVal = new List<Player>();
        while (reader.Read())
        {
            retVal.Add(Player.FromReader(reader));
        }
        return retVal;
    }
    public List<Game> GetGames()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT gameID,homeTeamID,awayTeamID,homeScore,awayScore,gameTime,venue FROM Games";
        var reader = sqlCommand.ExecuteReader();

        List<Game> retVal = new List<Game>();
        while (reader.Read())
        {
            retVal.Add(Game.FromReader(reader));
        }
        return retVal;
    }

    public List<Sport> GetSports()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT sportID,sportName FROM Sports";
        var reader = sqlCommand.ExecuteReader();

        List<Sport> retVal = new List<Sport>();
        while (reader.Read())
        {
            retVal.Add(Sport.FromReader(reader));
        }
        return retVal;

    }
    public List<Team> GetTeams()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT teamID,teamName,homeTown FROM Teams";
        var reader = sqlCommand.ExecuteReader();

        List<Team> retVal = new List<Team>();
        while (reader.Read())
        {
            retVal.Add(Team.FromReader(reader));
        }
        return retVal;
    }
    public List<Game> GetGamesByPlayer(Player player)
    {
        var games = GetGames();
        var playerMemberships = GetMembershipsByPlayer(player);
        var teams = GetTeams();

        var playerTeams = from team in teams where playerMemberships.Find(m => m.teamID == team.teamID) != null select team;
        var teamList = playerTeams.ToList();

        var playedGames = from game in games
                          where teamList.Find(t => (t.teamID == game.homeTeam) | (t.teamID == game.awayTeam)) != null
                          select game;
        return playedGames.ToList();
    }
    public List<TeamMembership> GetMembershipsByPlayer(Player player)
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT membershipID,playerID,teamID,season,jerseyNumber FROM TeamMemberships WHERE playerID = $playerID";
        sqlCommand.Parameters.AddWithValue("$playerID", player.playerID);
        var reader = sqlCommand.ExecuteReader();

        List<TeamMembership> retVal = new List<TeamMembership>();
        while (reader.Read())
        {
            retVal.Add(TeamMembership.FromReader(reader));
        }
        return retVal;
    }
    public List<TeamMembership> GetMembershipsByTeam(Team team)
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT membershipID,season,jerseyNumber FROM TeamMemberships WHERE teamID = $teamID";
        sqlCommand.Parameters.AddWithValue("$teamID", team.teamID);
        var reader = sqlCommand.ExecuteReader();

        List<TeamMembership> retVal = new List<TeamMembership>();
        while (reader.Read())
        {
            retVal.Add(TeamMembership.FromReader(reader));
        }
        return retVal;
    }

    public List<StatKind> GetStatKindsForSport(Sport sport)
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT statKindID,sportID,statName,unit FROM StatKinds WHERE sportID = $sportID";
        sqlCommand.Parameters.AddWithValue("$sportID", sport.sportID);

        var reader = sqlCommand.ExecuteReader();
        List<StatKind> retVal = new List<StatKind>();
        while (reader.Read())
        {
            retVal.Add(StatKind.FromReader(reader));
        }
        return retVal;

    }

    public List<StatInstance> GetStatInstancesForPlayer(Player player)
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT statInstanceID,playerID,gameID,statKindID,timestamp,value FROM StatInstances WHERE playerID = $playerID";
        sqlCommand.Parameters.AddWithValue($"playerID", player.playerID);

        var reader = sqlCommand.ExecuteReader();
        List<StatInstance> retVal = new List<StatInstance>();
        while (reader.Read())
        {
            retVal.Add(StatInstance.FromReader(reader));
        }
        return retVal;

    }

    public List<StatInstance> GetStatInstancesForGame(Game game)
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT statInstanceID,playerID,gameID,statKindID,timestamp,value FROM StatInstances WHERE gameID= $gameID";
        sqlCommand.Parameters.AddWithValue($"gameID", game.gameID);

        var reader = sqlCommand.ExecuteReader();
        List<StatInstance> retVal = new List<StatInstance>();
        while (reader.Read())
        {
            retVal.Add(StatInstance.FromReader(reader));
        }
        return retVal;
    }



}
