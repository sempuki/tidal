// tidal-rules: NAM-3
// tidal-expect: reject
// tidal-error-clang: redefinition of .total. with a different type
// tidal-error-gcc: conflicting declaration .double total
// tidal-error-edg: variable .total. has already been defined
// tidal-note: A class can share its name with one variable. Two variables with the name are still two declarations of one name.
struct total { int v; };
int total = 1;
double total = 2.0;
