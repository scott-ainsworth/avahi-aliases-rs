#!/bin/bash
##
# Test the `avahi-alias` program
##

[ "$1" !=  "debug" ] && [ "$1" != "release" ] &&
    echo "Usage: $(basename "$0") {debug|release}" &&
    exit 1

config="$1"

function run_test () {
    desc=$1
    expected=$2
    shift 2
    target/${config}/avahi-alias -f test-data/avahi-aliases $* 2>&1
    if diff -q test-data/$expected test-data/avahi-aliases >/dev/null 2>&1; then
        echo "test $desc ... ok"
    else
        echo "test $desc ... FAILED"
        echo "command: \"target/${config}/avahi-alias -f test-data/avahi-aliases $*\""
        echo "diff test-data/$expected test-data/avahi-aliases"
        diff --side-by-side test-data/$expected test-data/avahi-aliases
    fi
}

function run_error_test () {
    desc=$1
    expected=$2
    file=$3
    shift 3
    args=$*
    actual=$(target/${config}/avahi-alias -f test-data/$file $* 2>&1)
    if [ "$actual" == "$expected" ]; then
        echo "test $desc ... ok"
    else
        echo "test $desc ... FAILED"
        echo "command:  \"target/${config}/avahi-alias -f test-data/$file $*\""
        echo "expected: \"$expected\""
        echo "actual:   \"$actual\""
    fi
}

function run_list_test () {
    desc=$1
    expected=$2
    file=$3
    if target/${config}/avahi-alias -f test-data/$file list 2>&1 \
            | diff test-data/$expected - >/dev/null 2>&1; then
        echo "test $desc ... ok"
    else
        echo "test $desc ... FAILED"
        target/${config}/avahi-alias -f test-data/$file list 2>&1 \
            | diff --side-by-side test-data/$expected -
    fi
}

## VALID ALIASES TESTS

# Copy the test baseline
cp -f test-data/baseline-aliases test-data/avahi-aliases

# avahi-alias list tests
run_list_test "'avahi-alias list' lists aliases (no comments or blank lines)" \
              expected-list-result avahi-aliases
run_list_test "'avahi-alias list' flags invalid aliases" \
              expected-list-result-invalid-aliases baseline-invalid-aliases
run_error_test "'avahi-alias list' shows warning if no aliases" \
               $'No aliases in "test-data/baseline-empty-aliases"' \
               baseline-empty-aliases list

# avahi-alias add tests
run_test "'avahi-alias add' adds aliases to end of file" \
         expected-add-result add b1.local a1.local

# avahi-alias remove tests
run_test "'avahi-alias remove' removes aliases" \
         "expected-remove-result" remove a0.local a1.local b0.local

## INVALID ALIASES TESTS

# Copy the test baseline

cp -f test-data/baseline-invalid-aliases test-data/avahi-aliases

# avahi-alias add tests w/invalid aliases
run_error_test "'avahi-alias add' warns about invalid aliases" \
               $'Error: invalid alias "valerie.locl" found in "test-data/avahi-aliases"' \
               avahi-aliases add b1.local a1.local

# avahi-alias remove tests w/invalid aliases
run_error_test "'avahi-alias remove' warns about invalid aliases" \
               $'Error: invalid alias "valerie.locl" found in "test-data/avahi-aliases"' \
               avahi-aliases remove a0.local a1.local b0.local
run_test "'avahi-alias remove --force' removes invalid aliases" \
         expected-remove-force-result remove --force

# end
