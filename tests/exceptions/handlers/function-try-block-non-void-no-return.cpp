// tidal-rules: TRY-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: does not return a value in all control paths
// tidal-note: Clang warns by default; GCC gives no warning, also under -Wall. EDG warns by default and skips the test, which uses -Wall.
// tidal-flags: -Wall -Werror
int f() try { return 1; } catch (int) { }
int main() { return f(); }
