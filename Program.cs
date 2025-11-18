using System.Net;
using System.Net.WebSockets;
using System.Text;

HttpListener listener = new();
listener.Prefixes.Add("http://localhost:8080/");
listener.Start();
Console.WriteLine("WebSocket server running on ws://localhost:8080 ...");

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

            // TODO: Query your database here using `message`

            string response = $"Received: {message}";
            byte[] responseBytes = Encoding.UTF8.GetBytes(response);

            await socket.SendAsync(responseBytes, WebSocketMessageType.Text, true, CancellationToken.None);
        }
    }
    else
    {
        context.Response.StatusCode = 400;
        context.Response.Close();
    }
}
