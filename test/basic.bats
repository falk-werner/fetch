#!/usr/bin/env bats

setup() {
    FETCH=fetch
    if [[ -e target/debug/fetch ]] ; then
        FETCH=target/debug/fetch
    fi

    URL=https://github.com/falk-werner/fetch/archive/refs/tags/v0.1.0.tar.gz
    SHA256_HASH=bad3a69c413c049667ddd3e432dd2f7f9cdc30722059ec90577ab5fc901062e8
    MD5_HASH=c52c6e166e4eda3a053043d924f33797

    DATA_DIR=/tmp/fetch-test
    rm -rf $DATA_DIR
    mkdir -p $DATA_DIR
}

teardown() {
    rm -rf $DATA_DIR
}

@test "download file" {
    $FETCH -L $URL -o $DATA_DIR/out.bin 
    [[ -e $DATA_DIR/out.bin ]]
}

@test "download fails without redirect" {
    if $FETCH $URL -o $DATA_DIR/out.bin ; then
        false
    fi
    [[ ! -e $DATA_DIR/out.bin ]]
}

@test "check SHA256" {
    $FETCH -L $URL --sha256 $SHA256_HASH -o $DATA_DIR/out.bin 
    [[ -e $DATA_DIR/out.bin ]]
}

@test "check SHA256 failes due to invalid hash" {
    INVALID_HASH=baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaad
    if $FETCH -L $URL --sha256 $INVALID_HASH -o $DATA_DIR/out.bin ; then
        false
    fi
    [[ ! -e $DATA_DIR/out.bin ]]
}

@test "check MD5 of download" {
    $FETCH -L $URL --md5 $MD5_HASH -o $DATA_DIR/out.bin 
    [[ -e $DATA_DIR/out.bin ]]
}

@test "download failed with invalid MD5 hash" {
    INVALID_HASH=baaaaaaaaaaaaaaaaaaaaaaaaaaaaaad
    if $FETCH -L $URL --md5 $INVALID_HASH -o $DATA_DIR/out.bin ; then
        false
    fi
    [[ ! -e $DATA_DIR/out.bin ]]
}