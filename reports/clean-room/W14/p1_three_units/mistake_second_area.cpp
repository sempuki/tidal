// Deliberate mistake: a second definition of shapes::area in another unit.
// LNK-6 / ODR-8: exactly one definition in the program. The linker reports it.
#include "shapes.h"
int shapes::area(Rect r) { return r.width + r.height; }
