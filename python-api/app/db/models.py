from sqlalchemy.orm import declarative_base, Mapped, mapped_column
from sqlalchemy import DateTime, Numeric, Integer

Base = declarative_base()


class Price(Base):
    __tablename__ = "prices"

    id: Mapped[int] = mapped_column(Integer, primary_key=True, autoincrement=True)
    recorded_at: Mapped["datetime"] = mapped_column(DateTime(timezone=True), nullable=False)
    price: Mapped[float] = mapped_column(Numeric(12, 4), nullable=False)
