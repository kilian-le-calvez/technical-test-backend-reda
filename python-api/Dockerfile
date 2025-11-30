FROM python:3.12-slim

WORKDIR /app

COPY pyproject.toml .
RUN pip install --no-cache-dir .

COPY config.toml ./config.toml
COPY app ./app

ENV PYTHONUNBUFFERED=1

EXPOSE 8000
CMD ["python", "-m", "app.main"]
