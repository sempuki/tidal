// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: attempt to use a deleted function
// tidal-note: Q-0082. An array new destroys the elements it has built when a later one throws, so it needs the destructor. Clang rejects. GCC and EDG accept.
struct S { ~S() = delete; };
S* make() { return new S[2]; }
