from pydantic import BaseModel
from enum import Enum


class ErrorResponse(BaseModel):
    code: str
    message: str
class ErrorTemplate(BaseModel):
    detail: ErrorResponse

class ErrorEnum(Enum):
    INVALID_DATE_RANGE = (400, "Invalid date range")
    UPSTREAM_ERROR = (502, "Upstream service failure")
    TIMEOUT = (504, "Background computation timed out")

    def __init__(self, status_code: int, description: str):
        self.status_code = status_code
        self.description = description

def error_responses():
    return {
        err.status_code: {
            "model": ErrorTemplate,
            "description": err.description,
        }
        for err in ErrorEnum
    }