// tidal-rules: TYN-9
// tidal-expect: accept
// tidal-note: The unnamed class has a member class and a member enumeration that keep the limits, so the alias gives the class a name for linkage.
typedef struct { struct Part { int size; } part; enum Kind { small, large } kind; } Whole;
Whole make();
int use() { return make().part.size; }
