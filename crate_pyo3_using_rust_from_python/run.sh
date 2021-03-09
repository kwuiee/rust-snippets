set -e

cargo build --release
cd target/release
python -c 'import string_sum; print(string_sum.sum_as_string(1, 2))'
cd ../../
