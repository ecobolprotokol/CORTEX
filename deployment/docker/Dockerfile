FROM python:3.11-slim

WORKDIR /app

COPY pyproject.toml .
RUN pip install -e .

COPY src/ src/

EXPOSE 8080

CMD ["cortex", "serve"]
