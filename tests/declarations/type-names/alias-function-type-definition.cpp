// tidal-rules: TYN-2
// tidal-expect: reject
// tidal-error-edg: function type may not come from a typedef
// tidal-error-gcc: is initialized like a variable
using Fn = int(int);
Fn f { return 1; }
