// tidal-rules: ODR-15 ODR-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .rows
// tidal-error-gcc: redefinition of .int Table<int>::rows
// tidal-error-edg: has already been defined
// tidal-note: An explicit specialization of a static data member with an initializer is a definition, so a second one in the unit is not valid.
template <class T> struct Table { static int rows; };
template <> int Table<int>::rows = 1;
template <> int Table<int>::rows = 1;
