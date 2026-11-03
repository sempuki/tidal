// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .item
// tidal-error-gcc: item. does not name a type
// tidal-error-edg: variable .item. is not a type name
// tidal-note: The variable comes first and the class second. The variable still hides the class.
int item = 1;
struct item { int v; };
item first;
