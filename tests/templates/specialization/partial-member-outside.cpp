// tidal-rules: TSP-18
// tidal-expect: run
// tidal-note: A member of a partial specialization is defined outside with the partial specialization's head and argument list.
template <class T> struct Holder { int kind() const; };
template <class T> struct Holder<T*> { int kind() const; };
template <class T> int Holder<T>::kind() const { return 1; }
template <class T> int Holder<T*>::kind() const { return 2; }
int main() { return (Holder<int>().kind() == 1 && Holder<int*>().kind() == 2) ? 0 : 1; }
