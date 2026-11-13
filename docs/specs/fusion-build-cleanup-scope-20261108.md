# FUS-H001: respect cleanup command scope

The required post-build cleanup script currently checks `not args.temp_only` for both compiler artifacts and temporary fixtures, contrary to its CLI help. Thus `--target-only` still scans databases/indexes outside the build directory and `--temp-only` skips temporary cleanup entirely.

Required behavior: default invokes both categories; `--target-only` invokes compiler cleanup only; `--temp-only` invokes fixture cleanup only; specifying both is a usage error before cleanup. Retain existing helper behavior and existing dirty changes. This narrow change does not authorize broad fixture deletion during campaign testing: use `--target-only`; tests own their temporary data through RAII.

Make `strip = "debuginfo"` explicit in the root CI profile as mandated by AGENTS.md (already inherited from test). Keep dev/test profile debug=0 and incremental=false.

Verification: invoke `main` with patched cleanup functions to assert actual category dispatch and conflicting-option rejection, without touching real data. No product behavior or schema changes. Follow-up: orphaned library cleanup requires proving dependency reachability; age or shared crate-name alone is not enough to delete a distinct feature build.
