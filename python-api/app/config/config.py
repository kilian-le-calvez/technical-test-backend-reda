from dataclasses import dataclass
from pathlib import Path
import tomli

CONFIG_PATH = Path(__file__).resolve().parent.parent.parent / "config.toml"


@dataclass
class ServerConfig:
    host: str
    port: int


@dataclass
class DatabaseConfig:
    url: str


@dataclass
class CeleryConfig:
    broker_url: str
    result_backend: str
    task_default_queue: str
    result_expires_seconds: int


@dataclass
class AppConfig:
    server: ServerConfig
    database: DatabaseConfig
    celery: CeleryConfig


def load_config(path: Path = CONFIG_PATH) -> AppConfig:
    with path.open("rb") as f:
        raw = tomli.load(f)

    server = ServerConfig(**raw["server"])
    database = DatabaseConfig(**raw["database"])
    celery = CeleryConfig(**raw["celery"])
    return AppConfig(server=server, database=database, celery=celery)


config = load_config()
