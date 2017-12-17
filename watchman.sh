clear

watchman-make \
  -p 'src/bin/assembler.rs' --make "cargo" -t "test --bin syn-asm" -s 2\
  -p 'src/bin/disassembler.rs' --make "cargo" -t "test --bin syn-dis" -s 2\
  -p 'src/*.rs' -p 'src/**/*.rs' --make "cargo" -t "test --lib" -s 2 \
  -p 'src/*.rs' -p 'src/**/*.rs' --make "cargo" -t "build" -s 2 \
  -p 'Cargo.toml' --make "cargo" -t "update"


