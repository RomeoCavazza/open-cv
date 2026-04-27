# First stage - compilation
FROM maven:3.9.6-eclipse-temurin-21-alpine AS builder

WORKDIR /app

# Copy pom.xml and download dependencies
COPY pom.xml .
RUN mvn dependency:resolve

# Copy source code and build
COPY src ./src
RUN mvn package

# Second stage - run
FROM eclipse-temurin:21-jre-alpine

WORKDIR /app

# Copy the built JAR from builder stage
COPY --from=builder /app/target/worker-jar-with-dependencies.jar ./worker-jar-with-dependencies.jar

# Run the worker
CMD ["java", "-jar", "worker-jar-with-dependencies.jar"]

