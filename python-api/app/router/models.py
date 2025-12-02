from pydantic import BaseModel, Field
from datetime import date


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

