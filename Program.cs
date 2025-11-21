using System.Data.Common;
using System.Diagnostics;
using System.Net;
using System.Net.WebSockets;
using System.Text;
using System.Text.Json;
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
    process.StartInfo.FileName = "..\\..\\..\\src\\UI\\main.html";
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
                //added season selection, might want to update search and database to use season as well
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

            else if( queryData != null
                && queryData.TryGetValue("fetchSport", out string? fetchSport)
                )
            {
                // Fetch a list of sport names from the DB and return as JSON array
                var sportsList = dBConnection.FetchSports();
                byte[] sportsBytes = Encoding.UTF8.GetBytes(sportsList);
                await socket.SendAsync(sportsBytes, WebSocketMessageType.Text, true, CancellationToken.None);
                Console.WriteLine("Sent sports list to client.");
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
