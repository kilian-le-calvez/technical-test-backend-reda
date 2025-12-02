from celery import Celery
from app.config.config import config

celery_app = Celery("python-api-worker",include=["app.tasks.prices_tasks"])

celery_app.conf.update(
    broker_url=config.celery.broker_url,
    result_backend=config.celery.result_backend,
    task_default_queue=config.celery.task_default_queue,
    result_expires=config.celery.result_expires_seconds,
)
