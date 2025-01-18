#!/usr/bin/env bats

setup() {
    FETCH=fetch
    if [[ -e target/debug/fetch ]] ; then
        FETCH=target/debug/fetch
    fi

    pushd test-utils/test-server
    cargo build
    popd

    test-utils/test-server/target/debug/test-server &
    CHILD=$!
}

teardown() {
    kill $CHILD
}


@test "download fails due to timeout (--max-time)" {
    if $FETCH http://localhost:9000/slow_answer --max-time 5; then
        false
    fi
}