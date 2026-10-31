// tidal-rules: DTOR-3
// tidal-expect: reject
// tidal-error-clang: temporary of type .Sealed. has private destructor
// tidal-error-gcc: Sealed::~Sealed.*is private within this context
// tidal-error-edg: Sealed::~Sealed.*is inaccessible
class Sealed { ~Sealed(); public: Sealed(); };
Sealed make();
auto size = sizeof(make());
