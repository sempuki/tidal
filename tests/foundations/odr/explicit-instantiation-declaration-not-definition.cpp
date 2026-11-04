// tidal-rules: ODR-15
// tidal-expect: accept
// tidal-note: An explicit instantiation declaration of a variable template or of a static data member is not a definition, so the unit can repeat it.
template <class T> T zero = T();
template <class T> struct Table { static int rows; };
template <class T> int Table<T>::rows = 0;
extern template int zero<int>;
extern template int zero<int>;
extern template int Table<long>::rows;
extern template int Table<long>::rows;
