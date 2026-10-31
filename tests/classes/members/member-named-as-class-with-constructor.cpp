// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: has the same name as its class
// tidal-error-gcc: same name as (the )?(enclosing )?class
// tidal-error-edg: same name as its class
struct S { S(); int S; };
