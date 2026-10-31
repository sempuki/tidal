// tidal-rules: DTOR-3
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-note: Q-0082. Array new of a class with a deleted destructor; Clang rejects, GCC and EDG accept.
// tidal-note: Gap review 2026-10-31.
struct S { ~S() = delete; };
S* make() { return new S[2]; }
