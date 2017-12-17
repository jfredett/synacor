clear

if [ "$1" = builds ] ; then

  watchman-make \
    -p 'src/*.rs' -p 'src/**/*.rs' --make "clear; cargo" -t "build" -s 5 \
    -p 'Cargo.toml' --make "clear; cargo" -t "update"

elif [ "$1" = "tests" ] ; then

  watchman-make \
    -p 'src/bin/assembler.rs' --make "clear; cargo" -t "test --bin syn-asm" \
    -p 'src/bin/disassembler.rs' --make "clear; cargo" -t "test --bin syn-dis" \
    -p 'src/bin/vm.rs' --make "cargo" -t "test --bin syn-vm" \
    -p 'src/*.rs' -p 'src/**/*.rs' --make "clear; cargo" -t "test --lib"

fi


