from fastapi import APIRouter, HTTPException

from app.schemas.prices import AverageRequest, AverageResponse
from app.services.average_service import compute_average_for_range
from app.schemas.error import error_responses
router = APIRouter()


@router.post("/average-prices", response_model=AverageResponse,responses=error_responses())
def average_prices(payload: AverageRequest) -> AverageResponse:
    """
    Compute one average price per day between start_date and end_date (inclusive).
    """
    if payload.start_date > payload.end_date:
        raise HTTPException(
            status_code=400,
            detail={
                "code": "INVALID_DATE_RANGE",
                "message": "start_date must be <= end_date",
            },
        )

    try:
        items = compute_average_for_range(payload.start_date, payload.end_date)
    except TimeoutError as exc:
        raise HTTPException(status_code=504, detail={'code': 'TIMEOUT', 'message': 'Background computation timed out'}) from exc
    except Exception as exc:
        raise HTTPException(
            status_code=502,
            detail={
                "code": "UPSTREAM_ERROR",
                "message": "Failed to compute daily averages",
            },
        ) from exc

    return AverageResponse(data=items)
