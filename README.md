BOOTFROST - automated theorem proving program for first-order formulas with extensions. 

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

The formula consists of a tree of alternating typical quantifiers. Each typical quantifier is indicated on a new line with a mandatory indent (as in Python). The universal quantifier is denoted by the symbol "!", and the existential quantifier by the symbol "?". Typical existential and universal quantifiers that have a single immediate parent are combined into a disjunction and conjunction correspondingly. 

For a more comprehensive syntax description, please refer to the examples of formulas in the 'problems' subdirectory.

