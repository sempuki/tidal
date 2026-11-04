// tidal-rules: CE-19
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: cannot be narrowed to type .bool.
// tidal-error-gcc: narrowing conversion of .2. from .int. to .bool.
// tidal-note: Gap review 2026-10-31. Q-0078. EDG converts the condition to bool and permits narrowing.
struct S { explicit(2) S(int); };
