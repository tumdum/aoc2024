[private]
alias e := encrypt
[private]
alias p := prepare
[private]
alias f := fetch
[private]
alias s := show

show day:
    #!/usr/bin/env bash
    set -euxo pipefail
    cookie=`binarycookies -filter adventofcode.com ~/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies | rg session | sed 's/.* session \([^ ]*\).*/\1/g'`
    curl --silent  --cookie "session=$cookie" https://adventofcode.com/2024/day/{{ day }}| lynx -stdin --dump

fetch day:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=`printf "day%02d" {{ day }}`
    cookie=`binarycookies -filter adventofcode.com ~/Library/Containers/com.apple.Safari/Data/Library/Cookies/Cookies.binarycookies | rg session | sed 's/.* session \([^ ]*\).*/\1/g'`
    curl --silent --cookie "session=$cookie" 'https://adventofcode.com/2024/day/{{ day }}/input' > inputs/$name

solutions:
    cargo run --release -- --loops 10 --skip-output

encrypt day:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=`printf "day%02d" {{ day }}`
    rage -e -p inputs/$name -o inputs/$name.encrypted
    rm inputs/$name

prepare day:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=`printf "day%02d" {{ day }}`
    cp .day00 src/$name.rs
    git add src/$name.rs
    sed -i .bak "s/\/\/ pub mod $name;/pub mod $name;/g" src/lib.rs
    sed -i .bak "s/\/\/\(.*\)$name\(.*\)/\1$name\2/g" src/main.rs
    sed -i .bak "s/} \/\/,$name/, $name} \/\//g" benches/benchmarks.rs
    cargo fmt
