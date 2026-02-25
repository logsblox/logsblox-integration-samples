FROM maven:3.9.11-eclipse-temurin-21-alpine AS build
WORKDIR /src
COPY pom.xml ./
RUN mvn -q -DskipTests dependency:go-offline
COPY src ./src
RUN mvn -q -DskipTests package

FROM eclipse-temurin:21-jre-alpine
WORKDIR /app
COPY --from=build /src/target/java-otel-sdk.jar /app/app.jar
ENTRYPOINT ["java", "-jar", "/app/app.jar"]
