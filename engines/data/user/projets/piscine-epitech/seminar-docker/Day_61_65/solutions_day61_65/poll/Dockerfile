FROM python:3.11-slim

WORKDIR /app

# Copy requirements and install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application source code
COPY app.py wsgi.py ./
COPY templates ./templates
COPY static ./static

# Expose port 80
EXPOSE 80

# Run the application
CMD ["python", "app.py"]

