var builder = WebApplication.CreateBuilder(args);

builder.Services.AddHttpClient("client", cfg => {
    var hostName = Environment.GetEnvironmentVariable("SVC_API_HOSTNAME");
    var port = Environment.GetEnvironmentVariable("SVC_API_PORT");
    cfg.BaseAddress = new Uri($"http://{hostName}:{port}");
});

var app = builder.Build();

Environment.SetEnvironmentVariable("ASPNETCORE_URLS", "http://localhost:5000");

app.MapGet("/", async (IHttpClientFactory clientFactory, HttpContext context) => {
    var query = context.Request.Query["number"];
    var client = clientFactory.CreateClient("client");
    var response = await client.GetAsync($"/?number={query}");
    return await response.Content.ReadAsStringAsync();   
});

app.MapGet("/health", () => "Ok");


app.Run();
