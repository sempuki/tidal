// tidal-rules: OVL-9 OVL-35
// tidal-expect: reject
// tidal-error-clang: invalid argument type .Count. to unary expression
// tidal-error-gcc: no match for .operator!.
// tidal-error-edg: no operator .!. matches
// tidal-note: The explicit conversion function gives int, and int reaches bool by a boolean conversion. The built-in candidate takes an explicit conversion function only to bool, so ! has no viable function.
struct Count { explicit operator int() const; };
bool empty(Count c) { return !c; }
