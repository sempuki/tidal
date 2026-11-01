// tidal-rules: REF-10 REF-6
// tidal-expect: reject
// tidal-error-clang: could not bind to an lvalue
// tidal-error-gcc: cannot bind rvalue reference
// tidal-error-edg: an rvalue reference cannot be bound to an lvalue
// tidal-note: The braced literal has the referenced type, so the reference binds to the literal as = does. The literal is an lvalue, and an rvalue reference must not bind to it.
const char (&&word)[3] = {"ab"};
