// tidal-rules: CLS-26
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-A-1. In a class template, a member function with a ref-qualifier and one without, with the same parameters and different trailing requires clauses, are two functions, and the class is valid.
template <class T> concept Wide = sizeof(T) > 1;
template <class T> struct Holder {
  int get() & requires Wide<T> { return 1; }
  int get() requires (!Wide<T>) { return 2; }
};
Holder<int> wide;
Holder<char> narrow;
int a = wide.get();
int b = narrow.get();
