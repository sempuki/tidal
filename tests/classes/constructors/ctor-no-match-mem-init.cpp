// tidal-rules: CTOR-25
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Span.
// tidal-error-gcc: no matching function for call to .Span::Span.std::nullptr_t..
// tidal-error-edg: no instance of constructor .Span::Span. matches the argument list
struct Span { Span(int) { } };
struct Frame { Span span; Frame() : span(nullptr) { } };
