watchman-make \
  -p 'src/bin/assembler.rs' --make "clear ; cargo" -t "test --bin syn-asm" \
  -p 'src/bin/disassembler.rs' --make "clear ; cargo" -t "test --bin syn-dis" \
  -p 'src/*.rs' -p 'src/**/*.rs' --make "clear ; cargo" -t "test --lib"


