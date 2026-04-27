FROM node:21-alpine

WORKDIR /app

# Copy package files and install dependencies
COPY package.json .
RUN npm install --production

# Copy application source code
COPY server.js ./
COPY views ./views

# Expose port 80
EXPOSE 80

# Run the application
CMD ["node", "server.js"]

