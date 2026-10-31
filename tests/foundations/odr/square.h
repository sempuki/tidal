#ifndef TIDAL_ODR_SQUARE_H
#define TIDAL_ODR_SQUARE_H
struct Square {
  int side;
  int area() const { return side * side; }
};
inline int squares_made = 0;
inline const int* made_address() { return &squares_made; }
int perimeter(Square s);
const int* made_address_elsewhere();
#endif
