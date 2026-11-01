// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: functions that differ only in their return type cannot be overloaded
// tidal-error-gcc: ambiguating new declaration
// tidal-note: Two guides that are not templates have the same parameter types and different results. EDG accepts the declarations (Q-0280).
template <class T> struct Box { constexpr Box(T) {} };
Box(int) -> Box<long>;
Box(int) -> Box<short>;
