#!/bin/bash
OUTPUT_FILE=./.github/workflows/action.yml

cat >$OUTPUT_FILE << EOF
on:
  - push

jobs:
  main_test:
    name: Test changes to main
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v3

      - name: Install toolchain
        run: curl https://sh.rustup.rs -sSf | sh -s -- --profile minimal --default-toolchain stable -y && echo "\$HOME/.cargo/bin" >> \$GITHUB_PATH
      
      - name: Run cargo tests (meowgb-core)
        run: cargo test -p meowgb-core
EOF


for f in ./test-roms/blargg/roms/*
do
    f="${f##*/}"; f="${f%.*}";
    cat >>$OUTPUT_FILE << EOF
        
      - name: Run test ROM (blargg $f)
        if: always()
        run: cargo run -p meowgb-tests --release -- test-roms/blargg/roms/$f.gb test -m 100000000 -s meowgb-tests/expected_output/$f.bin
EOF
done

# for f in ./test-roms/mealybug-tearoom-tests/roms/*
# do
#     f="${f##*/}"; f="${f%.*}";
#     cat >>$OUTPUT_FILE << EOF

#       - name: Run test ROM (mealybug-tearoom-tests $f)
#         if: always()
#         run: cargo run -p meowgb-tests --release -- test-roms/mealybug-tearoom-tests/roms/$f.gb test -m 100000000 -s meowgb-tests/expected_output/$f.bin
# EOF
# done

for f in ./test-roms/mooneye-test-suite/roms/*
do
    f="${f##*/}"; f="${f%.*}";
    cat >>$OUTPUT_FILE << EOF

      - name: Run test ROM (mooneye-test-suite $f)
        if: always()
        run: cargo run -p meowgb-tests --release -- test-roms/mooneye-test-suite/roms/$f.gb test -m 100000000 -s meowgb-tests/expected_output/$f.bin
EOF
done
