from datetime import date

from app.tasks.prices_tasks import compute_daily_average
from app.schemas.prices import DailyAverageItem


def compute_average_for_range(start_date: date, end_date: date) -> list[DailyAverageItem]:
    """
    Orchestrates Celery task call and result handling.
    Synchronous pattern: wait for Celery task inside the request.
    """
    async_result = compute_daily_average.delay(
        start_date.isoformat(),
        end_date.isoformat(),
    )

    try:
        rows = async_result.get(timeout=10.0)
    except Exception as exc:
        raise exc

    return [DailyAverageItem(**row) for row in rows]
