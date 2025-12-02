from datetime import date
from typing import List

from pydantic import BaseModel, Field


class AverageRequest(BaseModel):
    start_date: date = Field(..., description="Start date (YYYY-MM-DD)")
    end_date: date = Field(..., description="End date (YYYY-MM-DD)")


class DailyAverageItem(BaseModel):
    date: date
    average_price: float


class AverageResponse(BaseModel):
    data: List[DailyAverageItem]
