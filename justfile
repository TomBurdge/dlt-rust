develop:
    uv run maturin develop --uv

rs_run: 
    uv run src/sample_payload.py

py_run:
    uv run chess_vanilla/chess_pipeline.py

release:
    uv run maturin build --release

benchmark: release
    time just rs_run
    time just py_run