// tidal-rules: NEW-11
// tidal-expect: reject
// tidal-error-clang: default initialization of an object of const type
// tidal-error-gcc: uninitialized const in .new.
// tidal-error-edg: const object requires an initializer
// tidal-note: Gap review 2026-10-31.
auto p = new const int;
