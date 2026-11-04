// shapes.h: declarations that the three units share (ODR-P3, INC-1).
// The include guard makes a second inclusion add no text (INC-4).
#ifndef W14_P1_SHAPES_H
#define W14_P1_SHAPES_H

namespace shapes {

// A class has one definition in each unit that needs it (ODR-7, ODR-10).
struct Rect {
  int width;
  int height;
};

// Functions with external linkage (LNK-1): declared here, defined once
// in the program (ODR-8).
int area(Rect r);
int perimeter(Rect r);

// An inline function can be defined in the header (LNK-4, INL-1).
inline bool is_square(Rect r) { return r.width == r.height; }

}  // namespace shapes

#endif
