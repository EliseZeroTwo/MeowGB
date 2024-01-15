echo "Running Test Roms..."

if ! cargo build -p meowgb-tests --release ; then
   exit
fi

TEST_TOTAL=0
TEST_SUCCESS=0

echo "Running test ROM ./test-roms/blargg/roms/cpu_instrs.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/blargg/roms/cpu_instrs.gb test -m 100000000 -s meowgb-tests/expected_output/blargg/cpu_instrs.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/blargg/roms/instr_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/blargg/roms/instr_timing.gb test -m 100000000 -s meowgb-tests/expected_output/blargg/instr_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/blargg/roms/mem_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/blargg/roms/mem_timing.gb test -m 100000000 -s meowgb-tests/expected_output/blargg/mem_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/add_sp_e_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/add_sp_e_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/add_sp_e_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/boot_hwio-dmgABCmgb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/boot_hwio-dmgABCmgb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/boot_hwio-dmgABCmgb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/boot_regs-dmgABC.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/boot_regs-dmgABC.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/boot_regs-dmgABC.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/call_cc_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/call_cc_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/call_cc_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/call_cc_timing2.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/call_cc_timing2.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/call_cc_timing2.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/call_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/call_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/call_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/call_timing2.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/call_timing2.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/call_timing2.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/di_timing-GS.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/di_timing-GS.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/di_timing-GS.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/div_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/div_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/div_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ei_sequence.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ei_sequence.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ei_sequence.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ei_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ei_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ei_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/halt_ime0_ei.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/halt_ime0_ei.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/halt_ime0_ei.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/halt_ime0_nointr_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/halt_ime0_nointr_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/halt_ime0_nointr_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/halt_ime1_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/halt_ime1_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/halt_ime1_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/halt_ime1_timing2-GS.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/halt_ime1_timing2-GS.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/halt_ime1_timing2-GS.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/if_ie_registers.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/if_ie_registers.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/if_ie_registers.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/intr_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/intr_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/intr_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/jp_cc_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/jp_cc_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/jp_cc_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/jp_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/jp_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/jp_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ld_hl_sp_e_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ld_hl_sp_e_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ld_hl_sp_e_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/oam_dma_restart.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/oam_dma_restart.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/oam_dma_restart.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/oam_dma_start.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/oam_dma_start.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/oam_dma_start.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/oam_dma_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/oam_dma_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/oam_dma_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/pop_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/pop_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/pop_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/push_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/push_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/push_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/rapid_di_ei.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/rapid_di_ei.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/rapid_di_ei.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ret_cc_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ret_cc_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ret_cc_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ret_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ret_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ret_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/reti_intr_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/reti_intr_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/reti_intr_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/reti_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/reti_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/reti_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/rst_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/rst_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/rst_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/bits_bank1.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/bits_bank1.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/bits_bank1.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/bits_bank2.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/bits_bank2.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/bits_bank2.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/bits_mode.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/bits_mode.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/bits_mode.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/bits_ramg.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/bits_ramg.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/bits_ramg.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/multicart_rom_8Mb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/multicart_rom_8Mb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/multicart_rom_8Mb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/ram_256kb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/ram_256kb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/ram_256kb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/ram_64kb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/ram_64kb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/ram_64kb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/rom_16Mb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/rom_16Mb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/rom_16Mb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/rom_1Mb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/rom_1Mb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/rom_1Mb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/rom_2Mb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/rom_2Mb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/rom_2Mb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/rom_4Mb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/rom_4Mb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/rom_4Mb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/rom_512kb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/rom_512kb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/rom_512kb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/MBC1/rom_8Mb.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/MBC1/rom_8Mb.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/MBC1/rom_8Mb.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/bits/mem_oam.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/bits/mem_oam.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/bits/mem_oam.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/bits/reg_f.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/bits/reg_f.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/bits/reg_f.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/bits/unused_hwio-GS.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/bits/unused_hwio-GS.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/bits/unused_hwio-GS.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/instr/daa.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/instr/daa.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/instr/daa.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/oam_dma/basic.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/oam_dma/basic.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/oam_dma/basic.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/oam_dma/reg_read.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/oam_dma/reg_read.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/oam_dma/reg_read.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ppu/intr_1_2_timing-GS.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ppu/intr_1_2_timing-GS.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ppu/intr_1_2_timing-GS.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ppu/intr_2_0_timing.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ppu/intr_2_0_timing.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ppu/intr_2_0_timing.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ppu/stat_irq_blocking.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ppu/stat_irq_blocking.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ppu/stat_irq_blocking.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ppu/stat_lyc_onoff.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ppu/stat_lyc_onoff.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ppu/stat_lyc_onoff.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/ppu/vblank_stat_intr-GS.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/ppu/vblank_stat_intr-GS.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/ppu/vblank_stat_intr-GS.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/div_write.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/div_write.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/div_write.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/rapid_toggle.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/rapid_toggle.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/rapid_toggle.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim00.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim00.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim00.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim00_div_trigger.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim00_div_trigger.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim00_div_trigger.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim01.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim01.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim01.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim01_div_trigger.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim01_div_trigger.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim01_div_trigger.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim10.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim10.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim10.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim10_div_trigger.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim10_div_trigger.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim10_div_trigger.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim11.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim11.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim11.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tim11_div_trigger.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tim11_div_trigger.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tim11_div_trigger.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tima_reload.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tima_reload.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tima_reload.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tima_write_reloading.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tima_write_reloading.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tima_write_reloading.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Running test ROM ./test-roms/mooneye-test-suite/roms/timer/tma_write_reloading.gb"

TEST_TOTAL=$((TEST_TOTAL + 1))

if res=$(./target/release/meowgb-tests test-roms/mooneye-test-suite/roms/timer/tma_write_reloading.gb test -m 100000000 -s meowgb-tests/expected_output/mooneye-test-suite/timer/tma_write_reloading.bin 2>&1 > /dev/null) ; then
  TEST_SUCCESS=$((TEST_SUCCESS + 1))
else
  echo "Failed: $res"
fi

echo "Succeeded in running $TEST_SUCCESS/$TEST_TOTAL"
