package simulations;

import io.gatling.javaapi.core.*;
import io.gatling.javaapi.http.*;

import static io.gatling.javaapi.core.CoreDsl.*;
import static io.gatling.javaapi.http.HttpDsl.*;

public class BasicSimulation extends Simulation {

    HttpProtocolBuilder httpProtocol = http
        .baseUrl("http://localhost:8000") // Altere conforme necessário
        .acceptHeader("application/json");

    ScenarioBuilder scn = scenario("Test Rust Rocket API")
        .exec(
            http("GET Root")
                .get("/")
                .check(status().is(200))
        );

    {
        setUp(
            scn.injectOpen(atOnceUsers(10))
        ).protocols(httpProtocol);
    }
}
