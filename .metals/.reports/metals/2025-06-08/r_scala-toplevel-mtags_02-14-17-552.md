error id: file://<HOME>/%C3%81rea%20de%20trabalho/std/devops/rust/docker_rocket/gatling/src/test/scala/com/api_rust_mvc/gatling/BasicSimulation.scala:[22..23) in Input.VirtualFile("file://<HOME>/%C3%81rea%20de%20trabalho/std/devops/rust/docker_rocket/gatling/src/test/scala/com/api_rust_mvc/gatling/BasicSimulation.scala", "package com.api_rust_.;gatling

import io.gatling.core.Predef._
import io.gatling.http.Predef._
import scala.concurrent.duration._

class BasicSimulation extends Simulation {

  val httpProtocol = http
    .baseUrl("http://localhost:8000") // URL base
    .acceptHeader("application/json")

  val scn = scenario("Basic GET Test")
    .exec(
      http("Get index")
        .get("/")
        .check(status.is(200))
    )

  setUp(
    scn.inject(atOnceUsers(10)) // 10 usuários simultâneos
  ).protocols(httpProtocol)
}
")
file://<HOME>/%C3%81rea%20de%20trabalho/std/devops/rust/docker_rocket/file:<HOME>/%25C3%2581rea%2520de%2520trabalho/std/devops/rust/docker_rocket/gatling/src/test/scala/com/api_rust_mvc/gatling/BasicSimulation.scala
file://<HOME>/%C3%81rea%20de%20trabalho/std/devops/rust/docker_rocket/gatling/src/test/scala/com/api_rust_mvc/gatling/BasicSimulation.scala:1: error: expected identifier; obtained semi
package com.api_rust_.;gatling
                      ^
#### Short summary: 

expected identifier; obtained semi