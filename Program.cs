using Microsoft.AspNetCore.Http;
using System;
using System.Data.Common;
using System.Diagnostics;
using System.Diagnostics.Metrics;
using System.Globalization;
using System.Net;
using System.Net.WebSockets;
using System.Reflection.Emit;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;
using System.Xml.Linq;
using static Search;

//start WebSocket server
HttpListener listener = new();
listener.Prefixes.Add("http://localhost:8080/");
listener.Start();
Console.WriteLine("WebSocket server running on ws://localhost:8080 ...");

//This relative path is from the executable location, so fix it if necessary when testing
StatsDB dBConnection = new StatsDB("./sqlite.db");

//open HTML interface in default browser
Process process = new Process();
try
{
    process.StartInfo.UseShellExecute = true;

    //This relative path is from the executable location, so fix it if necessary when testing
    process.StartInfo.FileName = "D:\\School\\CS 3560 - OOP\\ScoreKeeping\\src\\UI\\main.html";
    process.Start();
}
catch
{
    Console.WriteLine("Error opening HTML interface in browser.");
}

//open WebSocket connection and handle messages
while (true)
{
    HttpListenerContext context = await listener.GetContextAsync();

    if (context.Request.IsWebSocketRequest)
    {
        WebSocketContext wsContext = await context.AcceptWebSocketAsync(null);
        WebSocket socket = wsContext.WebSocket;
        Console.WriteLine("Client connected!");

        byte[] buffer = new byte[1024];

        while (socket.State == WebSocketState.Open)
        {
            WebSocketReceiveResult result = await socket.ReceiveAsync(buffer, CancellationToken.None);

            if (result.MessageType == WebSocketMessageType.Close)
            {
                Console.WriteLine("Client disconnected.");
                await socket.CloseAsync(WebSocketCloseStatus.NormalClosure, "", CancellationToken.None);
                break;
            }

            string message = Encoding.UTF8.GetString(buffer, 0, result.Count);
            Console.WriteLine($"Received: {message}");

            var queryData = System.Text.Json.JsonSerializer.Deserialize<Dictionary<string, string>>(message);
            if (queryData != null
                && queryData.TryGetValue("sport", out string? sport)
                && queryData.TryGetValue("season", out string? season)
                && queryData.TryGetValue("type", out string? type)
                && queryData.TryGetValue("query", out string? query)
                )
            {

                var searchType = Enum.Parse<SearchType>(type, true);
                var jsonResult = Search.BasicWebQuery(dBConnection, sport, searchType, query);
                byte[] jsonResultBytes = Encoding.UTF8.GetBytes(jsonResult ?? "");
                await socket.SendAsync(jsonResultBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent search result to client.");
            }

            else if (queryData != null
                && queryData.TryGetValue("fetchSport", out string? fetchSport)
                )
            {
                // Fetch a list of sport names from the DB and return as JSON array
                var sportsList = dBConnection.FetchSports();
                byte[] sportsBytes = Encoding.UTF8.GetBytes(sportsList);
                await socket.SendAsync(sportsBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent sports list to client.");
            }
            else if( queryData != null
                && queryData.TryGetValue("add", out string? addToDb)
                )
            {
                switch (addToDb)
                {
                    case "player":
                        if (queryData.TryGetValue("name", out string? playerName) &&
                            queryData.TryGetValue("dob", out string? dob) &&
                            queryData.TryGetValue("height", out string? height) &&
                            queryData.TryGetValue("weight", out string? weight) &&
                            DateTime.TryParseExact(dob, "yyyy-MM-dd", CultureInfo.InvariantCulture, DateTimeStyles.None, out DateTime playerDob) &&
                            int.TryParse(height, out int heightInt) &&
                            int.TryParse(weight, out int weightInt))
                        {
                            dBConnection.InsertPlayer(playerName, playerDob, heightInt, weightInt);
                        }
                        break;
                    case "team":
                        if (queryData.TryGetValue("name", out string? teamName) &&
                            queryData.TryGetValue("town", out string? town) &&
                            queryData.TryGetValue("sport", out string? teamSport) &&
                            (dBConnection.GetSportByName(teamSport) != null))
                        {
                            dBConnection.InsertTeam(dBConnection.GetSportByName(teamSport), teamName, town);
                        }
                        break;
                    case "sport":
                        if (queryData.TryGetValue("sport", out string? sportName))
                        {
                            if (sportName != "") dBConnection.InsertSport(sportName);
                        }
                        break;
                    case "game":
                        if (queryData.TryGetValue("hTeam", out string? homeTeam) &&
                            queryData.TryGetValue("aTeam", out string? awayTeam) &&
                            queryData.TryGetValue("hTeamScore", out string? homeTeamScore) &&
                            queryData.TryGetValue("aTeamScore", out string? awayTeamScore) &&
                            queryData.TryGetValue("time", out string? time) &&
                            queryData.TryGetValue("venue", out string? venue) &&
                            float.TryParse(homeTeamScore, out float hTeamScore) &&
                            float.TryParse(awayTeamScore, out float aTeamScore) &&
                            DateTime.TryParseExact(time, "yyyy-MM-dd", CultureInfo.InvariantCulture, DateTimeStyles.None, out DateTime gameTime))
                        {
                            dBConnection.InsertGame(dBConnection.GetTeamByName(homeTeam).getTeamID(), 
                                                    dBConnection.GetTeamByName(awayTeam).getTeamID(),
                                                    hTeamScore, aTeamScore, gameTime,venue);
                        }
                        break;
                    default:
                        break;
                }
            }
            if (queryData != null
                && queryData.TryGetValue("email", out string? email)
                && queryData.TryGetValue("password", out string? passwordHash)
                )
            {
                var userRole = Search.authenticate(dBConnection, email, passwordHash);
                byte[] stringResultBytes = Encoding.UTF8.GetBytes("" + userRole);
                await socket.SendAsync(stringResultBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent UserRole: " + userRole + " to client.");
            }
            if (queryData != null && queryData.TryGetValue("action", out string? action) && queryData.TryGetValue("player", out string? player))
            {

                if (action == "deleteRecord")
                {

                    // Pattern: looks for (ID, captures one or more digits, then looks for )
                    Match match = Regex.Match(player, @"\(ID\s(\d+)\)");

                    string idValue = "";

                    if (match.Success)
                    {
                        idValue = match.Groups[1].Value;

                    }
                    if (!string.IsNullOrEmpty(idValue))
                    {
                        int playerId = int.Parse(idValue);
                        dBConnection.RemovePlayer(playerId);
                    }
                }
            }
            /*string response = $"Received: {message}";
            byte[] responseBytes = Encoding.UTF8.GetBytes(response);

            await socket.SendAsync(responseBytes, WebSocketMessageType.Text, true, CancellationToken.None);
            */
        }
    }
    else
    {
        context.Response.StatusCode = 400;
        context.Response.Close();
    }
}
