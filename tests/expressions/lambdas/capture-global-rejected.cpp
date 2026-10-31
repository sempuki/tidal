// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-error-clang: does not have automatic storage duration
// tidal-error-gcc: with non-automatic storage duration
// tidal-error-edg: static storage duration cannot be captured
int global = 2;
auto g = [global] { return global; };
