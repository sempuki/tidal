// tidal-rules: DEP-25
// tidal-expect: accept
struct Has { template <class U> struct box { U value; }; };
template <class T> struct Outer {
  template <template <class> class Make = T::template box> struct In { Make<int> item; };
};
Outer<Has>::In<> in{{4}};
