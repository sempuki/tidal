// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .item
// tidal-error-gcc: expected .;. before .first
// tidal-error-edg: expected a .;.
// tidal-note: The global namespace declares the class and the variable. A use in a block finds the variable.
struct item { int v; };
int item = 1;
int main() { { item first{2}; return first.v; } }
