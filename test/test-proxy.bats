#!/usr/bin/env bats

setup() {
    FETCH=fetch
    if [[ -e target/debug/fetch ]] ; then
        FETCH=target/debug/fetch
    fi

    pushd test-utils/proxy-server
    cargo build
    popd

    test-utils/proxy-server/target/debug/proxy-server &
    CHILD=$!
}

teardown() {
    kill $CHILD
}

@test "http proxy" {
    data=$($FETCH -x http://localhost:7878/ http://www.google.de)
    [[ "$data" == "proxy-response" ]]
}
