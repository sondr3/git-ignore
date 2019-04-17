# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # due to caching there can be multiple versions of the man pages so we
    # remove the previous ones before building.
    rm -f target/$TARGET/release/build/git-ignore-generator-*/out/git-ignore.1

    cross rustc --bin git-ignore --target $TARGET --release -- -C lto

    cp target/$TARGET/release/git-ignore $stage/
    cp target/$TARGET/release/build/git-ignore-generator-*/out/git-ignore.1 $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
