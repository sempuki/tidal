// tidal-rules: TSP-22 TSP-3
// tidal-expect: run
// tidal-note: The member class Inner of Outer<int> is given explicitly. Its member is defined without template <>.
template <class T> struct Outer { struct Inner { int get(); }; };
template <> struct Outer<int>::Inner { int get(); };
int Outer<int>::Inner::get() { return 2; }
int main() { return Outer<int>::Inner{}.get() == 2 ? 0 : 1; }
