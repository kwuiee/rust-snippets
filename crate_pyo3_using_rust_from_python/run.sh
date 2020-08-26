maturin develop && \
python -c 'import rust2py; print(rust2py.__doc__); print(rust2py.sum_as_string(10, 20))'
pip uninstall -y rust2py
