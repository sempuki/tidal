// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Span.
// tidal-error-gcc: no matching function for call to .Span::Span
// tidal-error-edg: no instance of constructor .Span::Span. matches the argument list
// tidal-note: The implicit copy and move constructors are candidates, but Label converts to no parameter type.
struct Span { Span(int); };
struct Label {};
Span s{Label{}};
