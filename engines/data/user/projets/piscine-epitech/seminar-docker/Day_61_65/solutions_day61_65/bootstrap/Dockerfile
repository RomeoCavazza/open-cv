FROM debian:buster-slim

# Configure Debian sources to use archive (Buster is EOL)
RUN echo "deb http://archive.debian.org/debian buster main" > /etc/apt/sources.list && \
    echo "deb http://archive.debian.org/debian-security buster/updates main" >> /etc/apt/sources.list && \
    echo "Acquire::Check-Valid-Until false;" > /etc/apt/apt.conf.d/99no-check-valid-until

# Install Node.js and npm with apt-get
RUN apt-get update && \
    apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy the source code of the application into the image
COPY package.json app.js ./

# Install the application's dependencies with Node.js' package management tool
RUN npm install

# Expose port 3000
EXPOSE 3000

# Set the command to start the application
CMD ["node", "app.js"]
