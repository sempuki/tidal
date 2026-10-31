// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching conversion for functional-style cast from .std::nullptr_t. to .Span.
// tidal-error-gcc: no matching function for call to .Span::Span.std::nullptr_t..
// tidal-error-edg: no instance of constructor .Span::Span. matches the argument list
struct Span { Span(int) { } };
auto s = Span(nullptr);
