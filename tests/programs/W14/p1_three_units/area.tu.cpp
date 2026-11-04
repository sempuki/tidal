#include "shapes.h"
#include "shapes.h"   // a second inclusion adds nothing (INC-4)

// Definition outside the namespace with a qualified name (NS-6).
int shapes::area(Rect r) { return r.width * r.height; }
