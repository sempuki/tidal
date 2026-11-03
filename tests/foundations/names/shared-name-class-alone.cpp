// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .item
// tidal-error-gcc: item. does not name a type
// tidal-error-edg: variable .item. is not a type name
// tidal-note: The variable hides the class, so the name alone cannot declare an object of the class.
struct item { int v; };
int item = 1;
item first;
