// tidal-rules: LIST-2 CONV-21
// tidal-expect: reject
// tidal-error-clang: cannot be narrowed
// tidal-error-gcc: narrowing conversion of
// tidal-error-edg: invalid narrowing conversion
// tidal-note: bool is an integral type with two values. An int that is not a constant can hold other values, so the conversion narrows.
int i = 1;
bool b{i};
