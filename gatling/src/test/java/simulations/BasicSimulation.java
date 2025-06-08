package simulations;

import io.gatling.javaapi.core.*;
import io.gatling.javaapi.http.*;

import static io.gatling.javaapi.core.CoreDsl.*;
import static io.gatling.javaapi.http.HttpDsl.*;

public class BasicSimulation extends Simulation {

    HttpProtocolBuilder httpProtocol = http
        .baseUrl("http://localhost:8080")
        .acceptHeader("application/json")
        .contentTypeHeader("application/json");

    ScenarioBuilder scn = scenario("Test Actix API")
        // 1. Testa GET /hello
        .exec(
            http("GET /hello")
                .get("/hello")
                .check(status().is(200))
                .check(jsonPath("$.status").is("online"))
        )

        // 2. Testa GET /data
        .exec(
            http("GET /data")
                .get("/data")
                .check(status().is(200))
        )

        // 3. Testa POST /data
        .exec(
            http("POST /data")
                .post("/data")
                .body(StringBody("{ \"key\": \"gatling\", \"value\": \"test\" }")).asJson()
                .check(status().is(200))
                .check(substring("Salvo com sucesso"))
        );

    {
        setUp(
            scn.injectOpen(
                constantUsersPerSec(100).during(30)
            )
        ).protocols(httpProtocol);
    }
}
