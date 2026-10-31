// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Span.
// tidal-error-gcc: no matching function for call to .Span::Span
// tidal-error-edg: no instance of constructor .Span::Span. matches the argument list
struct Span { Span(int) { } Span(double, double) { } };
Span s{nullptr};
