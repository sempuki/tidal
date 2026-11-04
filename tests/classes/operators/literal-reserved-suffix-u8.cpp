// tidal-rules: OPR-8 OPR-9
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-note: Q-0383. The suffix u8 does not start with _, and an integer literal has no u8 form. GCC calls the literal operator. Clang rejects every literal with a reserved suffix, and EDG rejects this one, which starts with the letter of the u suffix.
// tidal-error-clang: invalid suffix .u8. on integer constant
// tidal-error-edg: extra text after expected end of number
int operator"" u8(unsigned long long v) { return int(v) + 100; }
int main() { return 1u8 == 101 ? 0 : 1; }
