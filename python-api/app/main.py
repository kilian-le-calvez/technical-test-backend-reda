from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
import logging

from app.router.router import api_router
from app.config.config import config
from app.schemas.error import ErrorResponse


logger = logging.getLogger(__name__)


app = FastAPI(
    title="python-api",
    version="1.0.0",
    docs_url="/swagger-ui",
    redoc_url=None,
    openapi_url="/openapi.json",
)

app.include_router(api_router, prefix="/internal")

@app.exception_handler(Exception)
async def generic_exception_handler(request: Request, exc: Exception):
    logger.exception(
        f"Unhandled error during request {request.method} {request.url}"
    )

    return JSONResponse(
        status_code=500,
        content=ErrorResponse(
            code="INTERNAL_ERROR",
            message="Internal server error",
        ).dict(),
    )




if __name__ == "__main__":
    import uvicorn

    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    )

    uvicorn.run(
        "app.main:app",
        host=config.server.host,
        port=config.server.port,
        reload=False,
    )