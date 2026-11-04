// tidal-rules: ODR-15
// tidal-expect: accept
// tidal-note: An explicit specialization of a static data member of a class template, or of a member variable template, with no initializer is a declaration only. The unit can repeat it, and the specialization with an initializer is the definition.
template <class T> struct Table {
  static int rows;
  template <class U> static U cell;
};
template <> int Table<int>::rows;
template <> int Table<int>::rows;
template <> int Table<int>::rows = 3;
template <> template <> long Table<int>::cell<long>;
template <> template <> long Table<int>::cell<long>;
