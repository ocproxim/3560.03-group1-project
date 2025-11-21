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
            
            if (queryData == null)
            {
                Console.WriteLine("Failed to parse message as JSON");
                continue;
            }

            // PRIORITY 1: Action handler (delete/update) - Check this FIRST!
            if (queryData.TryGetValue("action", out string? action))
            {
                bool success = false;
                string responseMessage = "";
                
                if (action == "deleteRecord" && queryData.TryGetValue("player", out string? player))
                {
                    Console.WriteLine($"DELETE request received for: {player}");
                    
                    // Pattern: looks for (ID, captures one or more digits, then looks for )
                    Match match = Regex.Match(player, @"\(ID\s*(\d+)\)");

                    if (match.Success)
                    {
                        string idValue = match.Groups[1].Value;
                        int playerId = int.Parse(idValue);
                        success = dBConnection.RemovePlayer(playerId);
                        
                        if (success)
                        {
                            responseMessage = $"Player ID {playerId} deleted successfully!";
                            Console.WriteLine(responseMessage);
                        }
                        else
                        {
                            responseMessage = $"Failed to delete player ID {playerId}";
                            Console.WriteLine(responseMessage);
                        }
                    }
                    else
                    {
                        responseMessage = "Failed to parse player ID from request";
                        Console.WriteLine(responseMessage);
                    }
                    
                    // Send response back to client
                    var deleteResponse = JsonSerializer.Serialize(new { success, message = responseMessage });
                    byte[] deleteResponseBytes = Encoding.UTF8.GetBytes(deleteResponse);
                    await socket.SendAsync(deleteResponseBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                    Console.WriteLine("Delete response sent to client");
                }
                else if (action == "updateRecord" && queryData.TryGetValue("oldName", out string? oldName) && queryData.TryGetValue("newName", out string? newName))
                {
                    Console.WriteLine($"UPDATE request received: {oldName} -> {newName}");
                    
                    // Pattern: looks for (ID, captures one or more digits, then looks for )
                    Match match = Regex.Match(oldName, @"\(ID\s*(\d+)\)");

                    if (match.Success)
                    {
                        string idValue = match.Groups[1].Value;
                        int playerId = int.Parse(idValue);
                        
                        // Get the player, update name, and save
                        var players = dBConnection.GetPlayers();
                        var player = players.Find(p => p.playerID == playerId);
                        
                        if (player != null)
                        {
                            player.setPlayerName(newName);
                            success = dBConnection.UpdatePlayer(player);
                            
                            if (success)
                            {
                                responseMessage = $"Player updated to '{newName}' successfully!";
                                Console.WriteLine(responseMessage);
                            }
                            else
                            {
                                responseMessage = "Failed to update player in database";
                                Console.WriteLine(responseMessage);
                            }
                        }
                        else
                        {
                            responseMessage = $"Player ID {playerId} not found";
                            Console.WriteLine(responseMessage);
                        }
                    }
                    else
                    {
                        responseMessage = "Failed to parse player ID from request";
                        Console.WriteLine(responseMessage);
                    }
                    
                    // Send response back to client
                    var updateResponse = JsonSerializer.Serialize(new { success, message = responseMessage });
                    byte[] updateResponseBytes = Encoding.UTF8.GetBytes(updateResponse);
                    await socket.SendAsync(updateResponseBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                    Console.WriteLine("Update response sent to client");
                }
                
                continue; // Skip to next message
            }
            
            // PRIORITY 2: Fetch sports list handler
            if (queryData.TryGetValue("fetchSport", out string? fetchSport))
            {
                Console.WriteLine("Fetch sports request received");
                var sportsList = dBConnection.FetchSports();
                byte[] sportsBytes = Encoding.UTF8.GetBytes(sportsList);
                await socket.SendAsync(sportsBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent sports list to client.");
                continue;
            }
            
            // PRIORITY 3: ADD handler
            if (queryData.TryGetValue("add", out string? addToDb))
            {
                Console.WriteLine($"ADD request received for: {addToDb}");
                bool success = false;
                string responseMessage = "";
                
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
                            success = true;
                            responseMessage = $"Player '{playerName}' added successfully!";
                            Console.WriteLine(responseMessage);
                        }
                        else
                        {
                            responseMessage = "Failed to add player - invalid data format";
                            Console.WriteLine(responseMessage);
                        }
                        break;
                        
                    case "team":
                        if (queryData.TryGetValue("name", out string? teamName) &&
                            queryData.TryGetValue("town", out string? town) &&
                            queryData.TryGetValue("sport", out string? teamSport))
                        {
                            var sportObj = dBConnection.GetSportByName(teamSport);
                            if (sportObj != null)
                            {
                                dBConnection.InsertTeam(sportObj, teamName, town);
                                success = true;
                                responseMessage = $"Team '{teamName}' added successfully!";
                                Console.WriteLine(responseMessage);
                            }
                            else
                            {
                                responseMessage = $"Sport '{teamSport}' not found. Please add the sport first.";
                                Console.WriteLine(responseMessage);
                            }
                        }
                        else
                        {
                            responseMessage = "Failed to add team - missing required fields";
                            Console.WriteLine(responseMessage);
                        }
                        break;
                        
                    case "sport":
                        if (queryData.TryGetValue("sport", out string? sportName) && !string.IsNullOrEmpty(sportName))
                        {
                            dBConnection.InsertSport(sportName);
                            success = true;
                            responseMessage = $"Sport '{sportName}' added successfully!";
                            Console.WriteLine(responseMessage);
                        }
                        else
                        {
                            responseMessage = "Failed to add sport - sport name is required";
                            Console.WriteLine(responseMessage);
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
                            var hTeam = dBConnection.GetTeamByName(homeTeam);
                            var aTeam = dBConnection.GetTeamByName(awayTeam);
                            
                            if (hTeam != null && aTeam != null)
                            {
                                dBConnection.InsertGame(hTeam.getTeamID(), aTeam.getTeamID(), hTeamScore, aTeamScore, gameTime, venue);
                                success = true;
                                responseMessage = $"Game '{homeTeam} vs {awayTeam}' added successfully!";
                                Console.WriteLine(responseMessage);
                            }
                            else
                            {
                                responseMessage = "One or both teams not found. Please add the teams first.";
                                Console.WriteLine(responseMessage);
                            }
                        }
                        else
                        {
                            responseMessage = "Failed to add game - invalid data format";
                            Console.WriteLine(responseMessage);
                        }
                        break;
                        
                    default:
                        responseMessage = $"Unknown add type: {addToDb}";
                        Console.WriteLine(responseMessage);
                        break;
                }
                
                // Send response back to client
                var addResponse = JsonSerializer.Serialize(new { success, message = responseMessage });
                byte[] addResponseBytes = Encoding.UTF8.GetBytes(addResponse);
                await socket.SendAsync(addResponseBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Add response sent to client");
                continue;
            }
            
            // PRIORITY 4: Authentication handler
            if (queryData.TryGetValue("email", out string? email) &&
                queryData.TryGetValue("password", out string? passwordHash))
            {
                Console.WriteLine($"Authentication request received for: {email}");
                var userRole = Search.authenticate(dBConnection, email, passwordHash);
                byte[] stringResultBytes = Encoding.UTF8.GetBytes("" + userRole);
                await socket.SendAsync(stringResultBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent UserRole: " + userRole + " to client.");
                continue;
            }
            
            // PRIORITY 5: Search query handler (most general, check LAST)
            if (queryData.TryGetValue("sport", out string? sport) &&
                queryData.TryGetValue("season", out string? season) &&
                queryData.TryGetValue("type", out string? type) &&
                queryData.TryGetValue("query", out string? query))
            {
                Console.WriteLine($"Search request received: {query} in {sport} ({type})");
                var searchType = Enum.Parse<SearchType>(type, true);
                var jsonResult = Search.BasicWebQuery(dBConnection, sport, searchType, query);
                byte[] jsonResultBytes = Encoding.UTF8.GetBytes(jsonResult ?? "");
                await socket.SendAsync(jsonResultBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent search result to client.");
                continue;
            }
            
            Console.WriteLine("Unknown message type received");
        }
    }
    else
    {
        context.Response.StatusCode = 400;
        context.Response.Close();
    }
}
