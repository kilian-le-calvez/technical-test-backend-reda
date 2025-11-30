from sqlalchemy import Column, Integer, Numeric, TIMESTAMP
from sqlalchemy.sql import func
from .db import Base


class Price(Base):
    __tablename__ = "prices"

    id = Column(Integer, primary_key=True, autoincrement=True)
    ts = Column(TIMESTAMP(timezone=True), nullable=False, server_default=func.now())
    price = Column(Numeric(18, 8), nullable=False)
