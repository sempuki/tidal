// tidal-rules: ACC-2
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class C { static int count; };
int x = C::count;
