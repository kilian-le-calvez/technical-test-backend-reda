from datetime import date, timedelta
from sqlalchemy import text
from celery import Celery
from .config import config

celery_app = Celery("python_api")
celery_app.conf.update(
    broker_url=config.celery.broker_url,
    result_backend=config.celery.result_backend,
    task_default_queue=config.celery.task_default_queue,
    result_expires=config.celery.result_expires_seconds,
)


@celery_app.task(name="compute_daily_average")
def compute_daily_average(start_date_str: str, end_date_str: str):
    from .db import engine

    start_date = date.fromisoformat(start_date_str)
    end_date = date.fromisoformat(end_date_str)
    end_date_exclusive = end_date + timedelta(days=1)
    query = text(
        """
        SELECT
            DATE(recorded_at ) AS day,
            AVG(price)::float8 AS avg_price
        FROM prices
        WHERE recorded_at  >= :start_date
          AND recorded_at  < :end_date_exclusive
        GROUP BY day
        ORDER BY day ASC
        """
    )

    with engine.connect() as conn:
        result = conn.execute(
            query,
            {
                "start_date": start_date,
                "end_date_exclusive": end_date_exclusive,
            },
        )
        rows = result.fetchall()

    return [
        {"date": row.day.isoformat(), "average_price": float(row.avg_price)}
        for row in rows
    ]
