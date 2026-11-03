// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: must use .enum. tag to refer to type .level
// tidal-error-gcc: level. does not name a type
// tidal-error-edg: variable .level. is not a type name
// tidal-note: The variable hides the enumeration, so the name alone cannot declare an object of the enumeration.
enum level { low, high };
int level = 2;
level current = low;
