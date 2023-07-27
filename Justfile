gather-coverage:
    #!/usr/bin/env bash
    RUSTFLAGS="-C instrument-coverage" \
    cargo test --tests
    xcrun llvm-profdata merge -sparse default_*.profraw -o your_crate.profdata
    xcrun llvm-cov report \
        $( \
        for file in \
            $( \
            RUSTFLAGS="-C instrument-coverage" \
                cargo test --tests --no-run --message-format=json \
                | jq -r "select(.profile.test == true) | .filenames[]" \
                | grep -v dSYM - \
            ); \
        do \
            printf "%s %s " -object $file; \
        done \
        ) \
    --instr-profile=your_crate.profdata --summary-only
    # llvm-cov show -Xdemangler=rustfilt target/debug/examples/formatjson5 \
    #     -instr-profile=your_crate.profdata \
    #     -show-line-counts-or-regions \
    #     -show-instantiations \
    #     -name=add_quoted_string

clean-coverage:
    rm -f *.profraw *.profdata
