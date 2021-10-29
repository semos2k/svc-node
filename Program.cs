var builder = WebApplication.CreateBuilder(args);

builder.Services.AddHttpClient("client", cfg => {
    var hostName = Environment.GetEnvironmentVariable("SVC_API_HOSTNAME") ?? "otrohost";
    var port = Environment.GetEnvironmentVariable("SVC_API_PORT") ?? "5000";
    cfg.BaseAddress = new Uri($"http://{hostName}:{port}");
});

var app = builder.Build();

app.MapGet("/", async (IHttpClientFactory clientFactory, HttpContext context) => {
    var query = context.Request.Query["number"];
    var client = clientFactory.CreateClient("client");
    var response = await client.GetAsync($"/?number={query}");
    return await response.Content.ReadAsStringAsync();   
});

app.MapGet("/healthz", () => "Ok");


app.Run("http://*:5000");
