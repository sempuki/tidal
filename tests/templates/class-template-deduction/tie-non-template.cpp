// tidal-rules: CTAD-22
// tidal-expect: accept
// tidal-note: Both guides take T for the one argument, and partial ordering ties. The guide from the constructor that is not a template wins. The guide from the explicit constructor template would make copy-list initialization fail (CTAD-23).
template <class T> struct Mark { constexpr Mark(T) {} template <class U = int> explicit constexpr Mark(T, U* = nullptr) {} };
Mark m = {1};
