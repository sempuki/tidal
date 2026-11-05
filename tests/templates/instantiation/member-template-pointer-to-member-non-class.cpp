// tidal-rules: INST-6 MPTR-8
// tidal-expect: reject
// tidal-error-clang: type 'int' cannot be used prior to '::'
// tidal-error-clang-18: member pointer refers into non-class type
// tidal-error-gcc: creating pointer to member of non-class type
// tidal-error-edg: name followed by "::" must be a class or namespace name
// tidal-note: The instantiation of Table<int> makes the declaration of the member template sort_by. Its parameter type is a pointer to member of int, which is not a class.
template <class Row> struct Table {
  template <class Key> void sort_by(Key Row::* key);
};
Table<int> t;
