from datetime import date
from typing import Iterable

from sqlalchemy import text
from sqlalchemy.engine import Connection, Row


def fetch_daily_averages(
    conn: Connection,
    start_date: date,
    end_exclusive: date,
) -> Iterable[Row]:
    """
    Perform optimized DB aggregation: compute average(price) per day.
    """
    query = text(
        """
        SELECT
            DATE(recorded_at) AS day,
            AVG(price)::float8 AS avg_price
        FROM prices
        WHERE recorded_at >= :start_date
          AND recorded_at < :end_exclusive
        GROUP BY day
        ORDER BY day ASC
        """
    )

    result = conn.execute(
        query,
        {
            "start_date": start_date,
            "end_exclusive": end_exclusive,
        },
    )
    return result.fetchall()