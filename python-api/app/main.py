from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from .routes import router as api_router
from .config import config

app = FastAPI(title="python-api", docs_url="/swagger-ui")

@app.exception_handler(Exception)
async def generic_exception_handler(request: Request, exc: Exception):
    return JSONResponse(
        status_code=500,
        content={"code": "INTERNAL_ERROR", "message": "Internal server error"},
    )

app.include_router(api_router)


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "app.main:app",
        host=config.server.host,
        port=config.server.port,
        reload=False,
    )
