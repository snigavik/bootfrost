cargo build

bootfrost --help

USAGE: \
    bootfrost [OPTIONS] --formula &lt;FORMULA&gt; --strategy &lt;STRATEGY&gt; --limit &lt;LIMIT&gt;

OPTIONS: \
    -f, --formula &lt;FORMULA&gt;      Path to the file containing the formula \
    -h, --help                   Print help information \
    -j, --json                   JSON logging \
    -l, --limit &lt;LIMIT&gt;          Maximum number of steps \
    -s, --strategy &lt;STRATEGY&gt;    Strategy: "plain", "general", "manualfirst", "manualbest" \
    -V, --version                Print version information

We recommend using the general strategy for automatic mode

Example: ./bootfrost -f ./problems/branch1.pcf -s general -l 1000



