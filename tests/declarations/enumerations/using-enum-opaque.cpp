// tidal-rules: ENUM-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-note: Gap review 2026-10-30. Q-0047. EDG accepts using enum on an enumeration that has only an opaque declaration; Clang and GCC reject it.
enum class C : int;
void f() { using enum C; }
