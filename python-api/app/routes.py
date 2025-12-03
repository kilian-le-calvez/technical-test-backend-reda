from fastapi import APIRouter, Depends, HTTPException, status
from pydantic import BaseModel, Field
from datetime import date
from .tasks import compute_daily_average

router = APIRouter()

# REVIEW: un fichier pour les models lié aux routes serait plus propre
class AverageRequest(BaseModel):
    # REVIEW: model_config = ConfigDict(extra="forbid")  # 🔒 whitelist
    # Prévisibilité, Documentation cohérente
    start_date: date = Field(..., description="Start date (YYYY-MM-DD)")
    end_date: date = Field(..., description="End date (YYYY-MM-DD)")


class DailyAverage(BaseModel):
    date: str
    average_price: float


class AverageResponse(BaseModel):
    data: list[DailyAverage]


class ErrorResponse(BaseModel):
    # REVIEW: Avoir un Enum pour les codes d'erreur
    code: str
    message: str


# REVIEW: Correspond pas au swagger, 422 et 200 manquants
# Pourquoi ne pas avoir remonté le 422 dans le rust jusqu'à l'utilisateur final ?
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
    # REVIEW: Result any type not safe and dev friendly
    async_result = compute_daily_average.delay(
        payload.start_date.isoformat(), payload.end_date.isoformat()
    )

    try:
        # REVIEW: Resulting here in an any function called
        rows = async_result.get(timeout=10)
    except Exception as exc:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail={"code": "TASK_FAILED", "message": f"Celery task failed: {exc}"},
        )

    # REVIEW: Maybe global type like ApiDataResponse & ApiErrorResponse
    # NOTE: Petit projet, pas over-engineer (la ya que une route) On a demandé de penser à beaucoup de choses pour un si petit projet
    return {"data": rows}

