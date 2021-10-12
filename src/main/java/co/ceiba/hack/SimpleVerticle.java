package co.ceiba.hack;

import io.vertx.core.AbstractVerticle;
import io.vertx.core.buffer.Buffer;
import io.vertx.ext.web.client.HttpResponse;
import io.vertx.ext.web.client.WebClient;

public class SimpleVerticle extends AbstractVerticle {
    private <T> T envVar(String name, T defaultValue){
        return System.getenv(name) == null ? defaultValue : (T)System.getenv(name);
    }

    @Override
    public void start() throws Exception {
        final String SVC_API_HOSTNAME = envVar("SVC_API_HOSTNAME", "localhost");
        final Integer SVC_API_PORT = Integer.parseInt(envVar("SVC_API_PORT", "4000"));

        System.out.println("endpoint: " + SVC_API_HOSTNAME + ":" + SVC_API_PORT);

        WebClient client = WebClient.create(vertx);

        vertx.createHttpServer().requestHandler(req -> {
            String path = req.path().toString();

            if( "/healthz".equals(path) ){
                req.response().setStatusCode(200).end("OK");
            }else {
                client.get(SVC_API_PORT, SVC_API_HOSTNAME, "/?" + req.query()).send(ar -> {
                    if (ar.succeeded()) {
                        HttpResponse<Buffer> response = ar.result();
                        String resp = response.body().toString("ISO-8859-1");

                        System.out.println(resp);
                        req.response().end(resp);
                    } else {
                        req.response().setStatusCode(500).end();
                        ar.cause().printStackTrace();
                    }
                });
            }
        }).listen(5000, listenResult -> {
            if (listenResult.failed()) {
                System.out.println("Could not start HTTP server");
                listenResult.cause().printStackTrace();
            } else {
                System.out.println("Server started");
            }
        });
    }
}
