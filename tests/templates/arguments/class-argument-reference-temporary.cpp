// tidal-rules: TARG-15
// tidal-expect: reject
// tidal-error-clang: reference to subobject of temporary object is not allowed in a template argument
// tidal-error-gcc: is not a constant expression
// tidal-error-edg: expression must have a constant value
// tidal-note: The reference member of the class argument binds to a member of a temporary.
struct Pair { int a; int b; };
struct Ref { const int& r; };
template <Ref R> struct Hold {};
Hold<Ref{Pair{1, 2}.a}> h;
