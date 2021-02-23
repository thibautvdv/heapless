set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features 'serde'

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        cargo test --test cpass --target $TARGET --features 'serde'
        cargo test --test cpass --target $TARGET --release --features 'serde'

        if [ $MSRV = 1 ]; then
            cd cfail
            cargo run
            cd ..
        fi


        if [ $TRAVIS_RUST_VERSION = nightly ]; then
            export RUSTFLAGS="-Z sanitizer=thread"
            export TSAN_OPTIONS="suppressions=$(pwd)/suppressions.txt"

            cargo test --test tsan --features x86-sync-pool --target $TARGET
            cargo test --test tsan --features x86-sync-pool --target $TARGET --release
        fi
    fi
}

# fake Travis variables to be able to run this on a local machine
if [ -z ${TRAVIS_BRANCH-} ]; then
    TRAVIS_BRANCH=auto
fi

if [ -z ${TRAVIS_PULL_REQUEST-} ]; then
    TRAVIS_PULL_REQUEST=false
fi

if [ -z ${TRAVIS_RUST_VERSION-} ]; then
    case $(rustc -V) in
        *nightly*)
            TRAVIS_RUST_VERSION=nightly
            ;;
        *beta*)
            TRAVIS_RUST_VERSION=beta
            ;;
        *)
            TRAVIS_RUST_VERSION=stable
            ;;
    esac
fi

if [ -z ${TARGET-} ]; then
    TARGET=$(rustc -Vv | grep host | cut -d ' ' -f2)
fi

if [ -z ${MSRV-} ]; then
    MSRV=0
fi

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
