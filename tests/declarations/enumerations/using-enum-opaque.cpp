// tidal-rules: ENUM-20
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: enumeration .C. is incomplete
// tidal-error-gcc: using enum. of .C. before its enum-specifier
// tidal-note: Gap review 2026-10-30. Q-0047. EDG accepts using enum on an enumeration that has only an opaque declaration; Clang and GCC reject it.
enum class C : int;
void f() { using enum C; }
