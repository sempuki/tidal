// tidal-rules: TSP-22 TSP-3
// tidal-expect: run
// tidal-note: The member class template Inner of Outer<int> is given explicitly as a class template. Its member's definition has template <> and then Inner's head.
template <class T> struct Outer { template <class U> struct Inner { int get(); }; };
template <> template <class U> struct Outer<int>::Inner { int get(); };
template <> template <class U> int Outer<int>::Inner<U>::get() { return 3; }
int main() { return Outer<int>::Inner<char>{}.get() == 3 ? 0 : 1; }
