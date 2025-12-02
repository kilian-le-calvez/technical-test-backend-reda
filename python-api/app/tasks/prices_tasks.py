from datetime import date, timedelta

from app.tasks.celery_app import celery_app
from app.db.session import get_engine
from app.db.queries import fetch_daily_averages

@celery_app.task(name="compute_daily_average")
def compute_daily_average(start_date_str: str, end_date_str: str) -> list[dict]:

    start = date.fromisoformat(start_date_str)
    end = date.fromisoformat(end_date_str)
    end_exclusive = end + timedelta(days=1)

    engine = get_engine()

    with engine.connect() as conn:
        rows = fetch_daily_averages(conn, start, end_exclusive)

    return [
        {"date": row.day.isoformat(), "average_price": float(row.avg_price)}
        for row in rows
    ]