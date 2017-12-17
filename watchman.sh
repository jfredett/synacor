watchman-make \
  -p 'src/bin/assembler.rs' --make "clear ; cargo" -t "test --bin syn-asm" -s 2\
  -p 'src/bin/disassembler.rs' --make "clear ; cargo" -t "test --bin syn-dis" -s 2\
  -p 'src/*.rs' -p 'src/**/*.rs' --make "clear ; cargo" -t "test --lib" -s 2 \
  -p 'src/*.rs' -p 'src/**/*.rs' --make "clear ; cargo" -t "build" -s 2 \
  -p 'Cargo.toml' --make "clear ; cargo" -t "update"


