from sqlalchemy import create_engine
from sqlalchemy.engine import Engine

from app.config.config import config

_engine: Engine | None = None


def get_engine() -> Engine:
    global _engine
    if _engine is None:
        _engine = create_engine(config.database.url, future=True, echo=False)
    return _engine
