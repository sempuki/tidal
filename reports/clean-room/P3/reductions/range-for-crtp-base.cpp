// tidal-fe stops: "unsupported at line 14: range-based for". g++ and clang++ accept, exit 0.
// With Base<int> in place of Base<R>, tidal-fe runs the program and returns 0.
template <class S> struct It {
    int i;
    It& operator++() { ++i; return *this; }
    int operator*() const { return i; }
    bool operator!=(It o) const { return i != o.i; }
};
template <class D> struct Base {
    It<D> begin() { return It<D>{0}; }
    It<D> end() { return It<D>{3}; }
};
struct R : Base<R> {};
int main() { R r; int s = 0; for (int x : r) s += x; return s - 3; }
