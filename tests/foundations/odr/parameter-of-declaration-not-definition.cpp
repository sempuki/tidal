// tidal-rules: ODR-15
// tidal-expect: accept
// tidal-note: A parameter in a function declaration that is not a definition is not a definition of a variable, so its type can be incomplete. The definition of report would need a complete type for its parameter.
struct Record;
void report(Record entry);
void report(Record other);
