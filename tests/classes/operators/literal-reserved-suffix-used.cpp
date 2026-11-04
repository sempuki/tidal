// tidal-rules: OPR-8 OPR-9
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-note: Q-0383. The suffix km does not start with _ and is not a form in the syntax of literals. GCC and EDG call the literal operator for km. Clang rejects the literal 1km and warns at the declaration that no literal will call the operator.
// tidal-error-clang: invalid suffix .km. on integer constant
int operator"" km(unsigned long long v) { return int(v) + 100; }
int main() { return 1km == 101 ? 0 : 1; }
