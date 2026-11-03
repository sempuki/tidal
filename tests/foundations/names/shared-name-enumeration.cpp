// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-note: An unscoped enumeration shares its name with a variable, and a scoped enumeration with a function.
enum level { low, high };
int level = 2;
enum class phase { start };
int phase();
