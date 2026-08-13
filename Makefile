.PHONY: setup test lint format build clean run serve

setup:
	pip install -e ".[dev]"

test:
	pytest tests/ -v --cov=cortex

test-unit:
	pytest tests/unit/ -v

test-integration:
	pytest tests/integration/ -v

test-security:
	pytest tests/security/ -v

lint:
	ruff check src/ tests/
	mypy src/

format:
	ruff format src/ tests/

build:
	python -m build

clean:
	rm -rf dist/ build/ *.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete

run:
	cortex run

serve:
	cortex serve

audit:
	python -m scripts.audit.security
	python -m scripts.audit.dependencies

release:
	python -m scripts.release.publish
