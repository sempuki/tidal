// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Span.
// tidal-error-gcc: no matching function for call to .Span::Span
// tidal-error-edg: no instance of constructor .Span::Span. matches the argument list
// tidal-note: The one argument is the list {1, 2}. Neither Span(int) nor the copy and move constructors can take it.
struct Span { Span(int); };
Span s{{1, 2}};
