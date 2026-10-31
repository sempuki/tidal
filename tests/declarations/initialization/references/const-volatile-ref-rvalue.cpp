// tidal-rules: REF-2
// tidal-expect: reject
// tidal-error-clang: volatile lvalue reference .* cannot bind to a temporary
// tidal-error-gcc: cannot bind lvalue reference of type .const volatile int&. to an rvalue
// tidal-error-edg: reference to const volatile must be an lvalue
// tidal-note: Gap review 2026-10-30.
const volatile int& r = 1;
