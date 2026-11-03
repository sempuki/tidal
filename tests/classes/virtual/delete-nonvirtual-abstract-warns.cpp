// tidal-rules: VIRT-13
// tidal-expect: reject
// tidal-error-clang: is abstract but has non-virtual destructor
// tidal-error-gcc: which has non-virtual destructor
// tidal-flags: -Wall -Werror
// tidal-note: The class is abstract, so the deleted object has a derived type.
struct Shape { virtual double area() const = 0; };
void drop(Shape* p) { delete p; }
