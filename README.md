cargo build

bootfrost --help

USAGE: \
    bootfrost [OPTIONS] --formula &lt; FORMULA &gt; --strategy `<`STRATEGY`>` --limit `<`LIMIT`>`

OPTIONS: \
    -f, --formula `<`FORMULA`>`      Path to the file containing the formula \
    -h, --help                   Print help information \
    -j, --json                   JSON logging \
    -l, --limit `<`LIMIT`>`          Maximum number of steps \
    -s, --strategy `<`STRATEGY`>`    Strategy: "plain", "general", "manualfirst", "manualbest" or path \
                                 to the file containing the user strategy \
    -V, --version                Print version information \

We recommend using the general strategy for automatic mode

Example: ./bootfrost -f ./problems/branch1.pcf -s general -l 1000

