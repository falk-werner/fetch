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

@test "connect insecure" {
    data=$($FETCH -k https://localhost:9000/)
    [[ "$data" == "Welcome!" ]]
}

@test "connection fails on insecure servers" {
    if $FETCH https://localhost:9000/ ; then
        false
    fi
}

@test "download fails due to timeout (--max-time)" {
    if $FETCH -k https://localhost:9000/slow_answer --max-time 5; then
        false
    fi
}

@test "post data" {
    data=$($FETCH -k https://localhost:9000/echo_post -d foobar)
    [[ "$data" == "foobar" ]]
}

@test "put data" {
    data=$($FETCH -k -X put https://localhost:9000/echo_put -d foobar)
    [[ "$data" == "foobar" ]]
}

@test "patch data" {
    data=$($FETCH -k -X patch https://localhost:9000/echo_patch -d foobar)
    [[ "$data" == "foobar" ]]
}

@test "post form data" {
    data=$($FETCH -k https://localhost:9000/echo_form -F name=Bob -F type=cat)
    echo "$data"
    [[ "$data" == "name = Bob;type = cat;" ]]
}

@test "delete data" {
    data=$($FETCH -k -X delete https://localhost:9000/delete )
    [[ "$data" == "Removed" ]]
}

@test "add http header" {
    data=$($FETCH -k https://localhost:9000/user_agent -H "User-Agent: spacebox")
    echo "$data"
    [[ "$data" == "spacebox" ]]
}
