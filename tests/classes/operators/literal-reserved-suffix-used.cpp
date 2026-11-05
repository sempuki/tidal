// tidal-rules: OPR-9
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-note: Q-0383. The suffix km does not start with _ and is not a form in the syntax of literals. OPR-9 needs no diagnostic, so the test checks only translation. GCC and EDG accept the literal 1km and call the literal operator. Clang rejects the literal and warns at the declaration that no literal will call the operator.
// tidal-error-clang: invalid suffix .km. on integer constant
int operator"" km(unsigned long long v) { return int(v) + 100; }
int main() { return 1km == 101 ? 0 : 1; }
