from fastapi import APIRouter

from app.router.routes_prices import router as prices_router

api_router = APIRouter()
api_router.include_router(prices_router, tags=["average"])