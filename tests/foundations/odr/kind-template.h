#ifndef TIDAL_ODR_KIND_TEMPLATE_H
#define TIDAL_ODR_KIND_TEMPLATE_H
template <class T> struct Kind {
  static int value() { return 0; }
};
template <class T> struct Kind<T*> {
  static int value() { return 1; }
  static int count;
};
template <class T> int Kind<T*>::count = 0;
template <> struct Kind<char> {
  static int value() { return 2; }
  static int count;
};
template <class T> int width(T) { return 4; }
template <> inline int width(char) { return 1; }
template <class T> int base_of = 10;
template <> inline int base_of<char> = 16;
int* pointer_count_elsewhere();
int* char_count_elsewhere();
int* char_base_elsewhere();
using CharFunction = int (*)(char);
CharFunction width_elsewhere();
#endif
