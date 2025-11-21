using Microsoft.Data.Sqlite;
using System.Text.Json;

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
    public bool RemoveGame(int gameID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM Games WHERE gameID = @gameID";
        command.Parameters.AddWithValue("@gameID", gameID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemovePlayer(int playerID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM Players WHERE playerID = @playerID";
        command.Parameters.AddWithValue("@playerID", playerID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemoveTeam(int teamID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM Teams WHERE teamID = @teamID";
        command.Parameters.AddWithValue("@teamID", teamID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemoveSport(int sportID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM Sports WHERE sportID = @sportID";
        command.Parameters.AddWithValue("@sportID", sportID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemoveStatKind(int statKindID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM StatKinds WHERE statKindID = @statKindID";
        command.Parameters.AddWithValue("@statKindID", statKindID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemoveStatInstance(int statInstanceID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM StatInstances WHERE statInstanceID = @statInstanceID";
        command.Parameters.AddWithValue("@statInstanceID", statInstanceID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemoveTeamMembership(int membershipID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM TeamMemberships WHERE membershipID = @membershipID";
        command.Parameters.AddWithValue("@membershipID", membershipID);

        return command.ExecuteNonQuery() > 0;
    }

    public bool RemoveUser(int userID)
    {
        var command = connection.CreateCommand();
        command.CommandText = "DELETE FROM Users WHERE userID = @userID";
        command.Parameters.AddWithValue("@userID", userID);

        return command.ExecuteNonQuery() > 0;
    }
    public bool UpdateGame(Game game)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE Games SET " +
                              "homeTeamID = @homeTeamID, " +
                              "awayTeamID = @awayTeamID, " +
                              "homeScore = @homeScore, " +
                              "awayScore = @awayScore, " +
                              "gameTime = @gameTime, " +
                              "venue = @venue " +
                              "WHERE gameID = @gameID";

        command.Parameters.AddWithValue("@gameID", game.gameID);
        command.Parameters.AddWithValue("@homeTeamID", game.homeTeamID);
        command.Parameters.AddWithValue("@awayTeamID", game.awayTeamID);
        command.Parameters.AddWithValue("@homeScore", game.homeScore);
        command.Parameters.AddWithValue("@awayScore", game.awayScore);
        command.Parameters.AddWithValue("@gameTime", game.gameTime);
        command.Parameters.AddWithValue("@venue", game.venue);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdatePlayer(Player player)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE Players SET " +
                              "name = @name, " +
                              "dateOfBirth = @dateOfBirth, " +
                              "height = @height, " +
                              "weight = @weight " +
                              "WHERE playerID = @playerID";

        command.Parameters.AddWithValue("@playerID", player.playerID);
        command.Parameters.AddWithValue("@name", player.name);
        command.Parameters.AddWithValue("@dateOfBirth", player.dateOfBirth);
        command.Parameters.AddWithValue("@height", player.height);
        command.Parameters.AddWithValue("@weight", player.weight);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdateTeam(Team team)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE Teams SET " +
                              "teamName = @teamName, " +
                              "homeTown = @homeTown " +
                              "WHERE teamID = @teamID";

        command.Parameters.AddWithValue("@teamID", team.teamID);
        command.Parameters.AddWithValue("@teamName", team.teamName);
        command.Parameters.AddWithValue("@homeTown", team.homeTown);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdateSport(Sport sport)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE Sports SET " +
                              "sportName = @sportName " +
                              "WHERE sportID = @sportID";

        command.Parameters.AddWithValue("@sportID", sport.sportID);
        command.Parameters.AddWithValue("@sportName", sport.sportName);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdateStatKind(StatKind statKind)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE StatKinds SET " +
                              "sportID = @sportID, " +
                              "statName = @statName, " +
                              "unit = @unit " +
                              "WHERE statKindID = @statKindID";

        command.Parameters.AddWithValue("@statKindID", statKind.statKindID);
        command.Parameters.AddWithValue("@sportID", statKind.sportID);
        command.Parameters.AddWithValue("@statName", statKind.statName);
        command.Parameters.AddWithValue("@unit", statKind.unit);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdateStatInstance(StatInstance statInstance)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE StatInstances SET " +
                              "playerID = @playerID, " +
                              "gameID = @gameID, " +
                              "statKindID = @statKindID, " +
                              "timestamp = @timestamp, " +
                              "value = @value " +
                              "WHERE statInstanceID = @statInstanceID";

        command.Parameters.AddWithValue("@statInstanceID", statInstance.statInstanceID);
        command.Parameters.AddWithValue("@playerID", statInstance.playerID);
        command.Parameters.AddWithValue("@gameID", statInstance.gameID);
        command.Parameters.AddWithValue("@statKindID", statInstance.statKindID);
        command.Parameters.AddWithValue("@timestamp", statInstance.timestamp);
        command.Parameters.AddWithValue("@value", statInstance.value);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdateTeamMembership(TeamMembership membership)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE TeamMemberships SET " +
                              "playerID = @playerID, " +
                              "teamID = @teamID, " +
                              "season = @season, " +
                              "jerseyNumber = @jerseyNumber " +
                              "WHERE membershipID = @membershipID";

        command.Parameters.AddWithValue("@membershipID", membership.membershipID);
        command.Parameters.AddWithValue("@playerID", membership.playerID);
        command.Parameters.AddWithValue("@teamID", membership.teamID);
        command.Parameters.AddWithValue("@season", membership.season);
        command.Parameters.AddWithValue("@jerseyNumber", membership.jerseyNumber);

        return command.ExecuteNonQuery() > 0;
    }

    public bool UpdateUser(User user)
    {
        var command = connection.CreateCommand();
        command.CommandText = "UPDATE Users SET " +
                              "email = @email, " +
                              "passwordHash = @passwordHash, " +
                              "role = @role " +
                              "WHERE userID = @userID";

        command.Parameters.AddWithValue("@userID", user.userID);
        command.Parameters.AddWithValue("@email", user.email);
        command.Parameters.AddWithValue("@passwordHash", user.passwordHash);
        command.Parameters.AddWithValue("@role", user.role);

        return command.ExecuteNonQuery() > 0;
    }
    public Game InsertGame(int homeID, int awayID, float homeScore, float awayScore, DateTime time, string venue)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO Games (homeTeamID, awayTeamID, homeScore, awayScore, gameTime, venue) " +
                              "VALUES (@homeTeamID, @awayTeamID, @homeScore, @awayScore, @gameTime, @venue); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@homeTeamID", homeID);
        command.Parameters.AddWithValue("@awayTeamID", awayID);
        command.Parameters.AddWithValue("@homeScore", homeScore);
        command.Parameters.AddWithValue("@awayScore", awayScore);
        command.Parameters.AddWithValue("@gameTime", time.ToShortDateString()); // Matching your original string format
        command.Parameters.AddWithValue("@venue", venue);

        long newID = (long)command.ExecuteScalar();


        return new Game((int)newID, homeID, awayID, homeScore, awayScore, time.ToShortDateString(), venue);
    }

    public Player InsertPlayer(string name, DateTime dateOfBirth, int height, int weight)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO Players (name, dateOfBirth, height, weight) " +
                              "VALUES (@name, @dateOfBirth, @height, @weight); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@name", name);
        command.Parameters.AddWithValue("@dateOfBirth", dateOfBirth);
        command.Parameters.AddWithValue("@height", height);
        command.Parameters.AddWithValue("@weight", weight);

        long newID = (long)command.ExecuteScalar();

        return new Player((int)newID, name, dateOfBirth.ToShortDateString(), height, weight);
    }

    public Team InsertTeam(Sport sport, string teamName, string homeTown)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO Teams (teamName, homeTown) " +
                              "VALUES (@teamName, @homeTown); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@teamName", teamName);
        command.Parameters.AddWithValue("@homeTown", homeTown);

        long newID = (long)command.ExecuteScalar();

        return new Team((int)newID, sport.sportID, teamName, homeTown);
    }

    public Sport InsertSport(string sportName)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO Sports (sportName) " +
                              "VALUES (@sportName); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@sportName", sportName);

        long newID = (long)command.ExecuteScalar();

        return new Sport((int)newID, sportName);
    }

    public StatKind InsertStatKind(int sportID, string statName, string unit)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO StatKinds (sportID, statName, unit) " +
                              "VALUES (@sportID, @statName, @unit); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@sportID", sportID);
        command.Parameters.AddWithValue("@statName", statName);
        command.Parameters.AddWithValue("@unit", unit);

        long newID = (long)command.ExecuteScalar();

        return new StatKind((int)newID, sportID, statName, unit);
    }

    public StatInstance InsertStatInstance(int playerID, int gameID, int statKindID, DateTime timestamp, float value)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO StatInstances (playerID, gameID, statKindID, timestamp, value) " +
                              "VALUES (@playerID, @gameID, @statKindID, @timestamp, @value); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@playerID", playerID);
        command.Parameters.AddWithValue("@gameID", gameID);
        command.Parameters.AddWithValue("@statKindID", statKindID);
        command.Parameters.AddWithValue("@timestamp", timestamp);
        command.Parameters.AddWithValue("@value", value);

        long newID = (long)command.ExecuteScalar();

        return new StatInstance((int)newID, playerID, gameID, statKindID, timestamp.ToLongTimeString(), value);
    }

    public TeamMembership InsertTeamMembership(int playerID, int teamID, string season, int jerseyNumber)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO TeamMemberships (playerID, teamID, season, jerseyNumber) " +
                              "VALUES (@playerID, @teamID, @season, @jerseyNumber); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@playerID", playerID);
        command.Parameters.AddWithValue("@teamID", teamID);
        command.Parameters.AddWithValue("@season", season);
        command.Parameters.AddWithValue("@jerseyNumber", jerseyNumber);

        long newID = (long)command.ExecuteScalar();

        return new TeamMembership((int)newID, playerID, teamID, season, jerseyNumber);
    }

    public User InsertUser(string email, string passwordHash, UserRole role)
    {
        var command = connection.CreateCommand();
        command.CommandText = "INSERT INTO Users (email, passwordHash, role) " +
                              "VALUES (@email, @passwordHash, @role); " +
                              "SELECT last_insert_rowid();";

        command.Parameters.AddWithValue("@email", email);
        command.Parameters.AddWithValue("@passwordHash", passwordHash);
        command.Parameters.AddWithValue("@role", role);

        long newID = (long)command.ExecuteScalar();

        return new User((int)newID, email, passwordHash, role);
    }
    public List<User> GetUsers()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT userID,email,passwordHash,role FROM Users";
        var reader = sqlCommand.ExecuteReader();

        List<User> retVal = new List<User>();
        while (reader.Read())
        {
            retVal.Add(User.FromReader(reader));
        }
        return retVal;

    }

    public List<TeamMembership> GetTeamMemberships()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT membershipID,playerID,teamID,season,jerseyNumber FROM TeamMemberships";
        var reader = sqlCommand.ExecuteReader();

        List<TeamMembership> retVal = new List<TeamMembership>();
        while (reader.Read())
        {
            retVal.Add(TeamMembership.FromReader(reader));
        }
        return retVal;

    }

    public List<StatInstance> GetStatInstances()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT statInstanceID,playerID,gameID,statKindID,timestamp,value FROM StatInstances";

        var reader = sqlCommand.ExecuteReader();
        List<StatInstance> retVal = new List<StatInstance>();
        while (reader.Read())
        {
            retVal.Add(StatInstance.FromReader(reader));
        }
        return retVal;

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
    public List<Player> GetPlayersByTeam(Team team)
    {
        var players = GetPlayers();
        var memberships = GetMembershipsByTeam(team);

        var filtered = from player in players where memberships.Find(m => m.playerID == player.playerID) != null select player;

        return filtered.ToList();

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
        sqlCommand.CommandText = "SELECT teamID,sportID,teamName,homeTown FROM Teams";
        var reader = sqlCommand.ExecuteReader();

        List<Team> retVal = new List<Team>();
        while (reader.Read())
        {
            retVal.Add(Team.FromReader(reader));
        }
        return retVal;
    }
    public List<Team> GetTeamsBySport(Sport sport)
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT teamID,sportID,teamName,homeTown FROM Teams WHERE sportID = $sportID";
        sqlCommand.Parameters.AddWithValue("$sportID", sport.sportID);
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
                          where teamList.Find(t => (t.teamID == game.homeTeamID) | (t.teamID == game.awayTeamID)) != null
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
        sqlCommand.CommandText = "SELECT membershipID,playerID,teamID,season,jerseyNumber FROM TeamMemberships WHERE teamID = $teamID";
        sqlCommand.Parameters.AddWithValue("$teamID", team.teamID);
        var reader = sqlCommand.ExecuteReader();

        List<TeamMembership> retVal = new List<TeamMembership>();
        while (reader.Read())
        {
            retVal.Add(TeamMembership.FromReader(reader));
        }
        return retVal;
    }
    public List<StatKind> GetStatKinds()
    {
        var sqlCommand = connection.CreateCommand();
        sqlCommand.CommandText = "SELECT statKindID,sportID,statName,unit FROM StatKinds";
        var reader = sqlCommand.ExecuteReader();
        List<StatKind> retVal = new List<StatKind>();
        while (reader.Read())
        {
            retVal.Add(StatKind.FromReader(reader));
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

}
