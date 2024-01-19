#!/bin/zsh
GH_ACTION_OUTPUT_FILE=./.github/workflows/action.yml
FJ_ACTION_OUTPUT_FILE=./.forgejo/workflows/action.yml
TEST_SCRIPT_OUTPUT_FILE=./run-test-roms.sh
TEST_MD_FILE=./tests.md

cat >$TEST_MD_FILE << EOF
# Passing Tests

EOF

cat >$TEST_SCRIPT_OUTPUT_FILE << EOF
echo "Running Test Roms..."

if ! cargo build -p meowgb-tests --release ; then
   exit
fi

TEST_TOTAL=0
TEST_SUCCESS=0
EOF

chmod +x $TEST_SCRIPT_OUTPUT_FILE


tee $GH_ACTION_OUTPUT_FILE $FJ_ACTION_OUTPUT_FILE >/dev/null << EOF
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

      - name: Build release (meowgb-tests)
        run: cargo build -p meowgb-tests --release
EOF

cat >>$TEST_MD_FILE << EOF
## Blargg's Test ROMs

EOF

for full_f in ./test-roms/blargg/serial-roms/*.gb
do
    f="${full_f##*/}"; f="${f%.*}";
    TEST_CMD="./target/release/meowgb-tests test-roms/blargg/serial-roms/$f.gb test-serial -m 100000000 -s meowgb-tests/expected_output/serial/blargg/$f.bin"

    cat >>$TEST_SCRIPT_OUTPUT_FILE << EOF

echo "Running test ROM $full_f"

TEST_TOTAL=\$((TEST_TOTAL + 1))

if res=\$($TEST_CMD 2>&1 > /dev/null) ; then
  TEST_SUCCESS=\$((TEST_SUCCESS + 1))
else
  echo "Failed: \$res"
fi
EOF

    cat >>$TEST_MD_FILE << EOF
* $f.gb - [ROM]($full_f) - [Expected Serial Output](./meowgb-tests/expected_output/serial/blargg/$f.bin)
EOF

    tee -a $GH_ACTION_OUTPUT_FILE $FJ_ACTION_OUTPUT_FILE >/dev/null << EOF
        
      - name: Run test ROM (blargg $f)
        if: always()
        run: $TEST_CMD
EOF
done

cat >>$TEST_MD_FILE << EOF

## Mooneye Test Suite

EOF

for full_f in ./test-roms/mooneye-test-suite/serial-roms/*.gb
do
    f="${full_f##*/}"; f="${f%.*}";
      TEST_CMD="./target/release/meowgb-tests test-roms/mooneye-test-suite/serial-roms/$f.gb test-serial -m 100000000 -s meowgb-tests/expected_output/serial/mooneye-test-suite/$f.bin"

    cat >>$TEST_SCRIPT_OUTPUT_FILE << EOF

echo "Running test ROM $full_f"

TEST_TOTAL=\$((TEST_TOTAL + 1))

if res=\$($TEST_CMD 2>&1 > /dev/null) ; then
  TEST_SUCCESS=\$((TEST_SUCCESS + 1))
else
  echo "Failed: \$res"
fi
EOF

    cat >>$TEST_MD_FILE << EOF
* $f.gb - [ROM]($full_f) - [Expected Serial Output](./meowgb-tests/expected_output/serial/mooneye-test-suite/$f.bin)
EOF

    tee -a $GH_ACTION_OUTPUT_FILE $FJ_ACTION_OUTPUT_FILE >/dev/null << EOF

      - name: Run test ROM (mooneye-test-suite $f)
        if: always()
        run: $TEST_CMD
EOF
done

for directory in ./test-roms/mooneye-test-suite/serial-roms/*/
do
  d=$(basename $directory)

  cat >>$TEST_MD_FILE << EOF

### $d

EOF

  for full_f in ./test-roms/mooneye-test-suite/serial-roms/$d/*.gb
  do
    f="${full_f##*/}"; f="${f%.*}";
    TEST_CMD="./target/release/meowgb-tests test-roms/mooneye-test-suite/serial-roms/$d/$f.gb test-serial -m 100000000 -s meowgb-tests/expected_output/serial/mooneye-test-suite/$d/$f.bin"

    cat >>$TEST_SCRIPT_OUTPUT_FILE << EOF

echo "Running test ROM $full_f"

TEST_TOTAL=\$((TEST_TOTAL + 1))

if res=\$($TEST_CMD 2>&1 > /dev/null) ; then
  TEST_SUCCESS=\$((TEST_SUCCESS + 1))
else
  echo "Failed: \$res"
fi
EOF

    cat >>$TEST_MD_FILE << EOF
* $d/$f.gb - [ROM]($full_f) - [Expected Serial Output](./meowgb-tests/expected_output/serial/mooneye-test-suite/$d/$f.bin)
EOF

    tee -a $GH_ACTION_OUTPUT_FILE $FJ_ACTION_OUTPUT_FILE >/dev/null << EOF

      - name: Run test ROM (mooneye-test-suite $d/$f)
        if: always()
        run: $TEST_CMD
EOF
  done

done

cat >>$TEST_MD_FILE << EOF

## Hacktix Test ROMs

EOF

for full_f in ./test-roms/hacktix/framebuffer-roms/*.gb
do
    f="${full_f##*/}"; f="${f%.*}";
    TEST_CMD="./target/release/meowgb-tests test-roms/hacktix/framebuffer-roms/$f.gb test-framebuffer -m 100000000 -s meowgb-tests/expected_output/framebuffer/hacktix/$f.bin"

    cat >>$TEST_SCRIPT_OUTPUT_FILE << EOF

echo "Running test ROM $full_f"

TEST_TOTAL=\$((TEST_TOTAL + 1))

if res=\$($TEST_CMD 2>&1 > /dev/null) ; then
  TEST_SUCCESS=\$((TEST_SUCCESS + 1))
else
  echo "Failed: \$res"
fi
EOF

    cat >>$TEST_MD_FILE << EOF
* $f.gb - [ROM]($full_f) - [Expected Framebuffer (RGBA32)](./meowgb-tests/expected_output/framebuffer/hacktix/$f.bin)
EOF

    tee -a $GH_ACTION_OUTPUT_FILE $FJ_ACTION_OUTPUT_FILE >/dev/null << EOF
        
      - name: Run test ROM (hacktix $f)
        if: always()
        run: $TEST_CMD
EOF
done

cat >>$TEST_SCRIPT_OUTPUT_FILE << EOF

echo "Succeeded in running \$TEST_SUCCESS/\$TEST_TOTAL"
EOF
