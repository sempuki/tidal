// tidal-fe rejects with NAM-9 at line 4. g++ and clang++ accept, exit 0.
// Base<Derived>::begin is never called, so its definition is never needed.
// Declaring begin() without a body is accepted by tidal-fe.
template <class S> struct Cursor { typename S::value_type current; };
template <class D> struct Base { Cursor<D> begin() { return Cursor<D>{}; } };
struct Derived : Base<Derived> { using value_type = int; };
int main() { Derived d; (void)d; return 0; }
