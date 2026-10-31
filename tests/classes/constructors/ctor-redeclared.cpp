// tidal-rules: CTOR-1
// tidal-expect: reject
// tidal-error-clang: constructor cannot be redeclared
// tidal-error-gcc: cannot be overloaded with
// tidal-error-edg: invalid redeclaration of member function
struct S { S(); S(); };
