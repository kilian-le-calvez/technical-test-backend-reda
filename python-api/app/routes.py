from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel, Field
from datetime import date
from .tasks import compute_daily_average

router = APIRouter()


class AverageRequest(BaseModel):
    start_date: date = Field(..., description="Start date (YYYY-MM-DD)")
    end_date: date = Field(..., description="End date (YYYY-MM-DD)")


class DailyAverage(BaseModel):
    date: str
    average_price: float


class AverageResponse(BaseModel):
    data: list[DailyAverage]


class ErrorResponse(BaseModel):
    code: str
    message: str


@router.post(
    "/internal/average-prices",
    response_model=AverageResponse,
    responses={400: {"model": ErrorResponse}, 500: {"model": ErrorResponse}},
)
def average_prices(payload: AverageRequest):
    if payload.start_date > payload.end_date:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail={"code": "INVALID_DATE_RANGE", "message": "start_date must be <= end_date"},
        )

    # Start Celery task and wait synchronously for result.
    # Rationale (for documentation):
    # - Keeps external API stateless for the Rust client.
    # - Simple to reason about; latency is dominated by a single DB aggregation.
    # - If needed, this can be changed to async/result retrieval endpoint later.
    async_result = compute_daily_average.delay(
        payload.start_date.isoformat(), payload.end_date.isoformat()
    )

    try:
        rows = async_result.get(timeout=10)
    except Exception as exc:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail={"code": "TASK_FAILED", "message": f"Celery task failed: {exc}"},
        )

    return {"data": rows}
