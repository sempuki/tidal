// tidal-rules: CTOR-25
// tidal-expect: accept
// tidal-note: Each initialization with one argument of Span, or of a class that converts to Span, uses the implicit copy or move constructor.
struct Span { Span(int); };
struct Maker { operator Span() const; };
Span a{1};
Span b{a};
Span c{Span{2}};
Span d{Maker{}};
Span e(Maker{});
Span f({1});
Span g{{1}};
