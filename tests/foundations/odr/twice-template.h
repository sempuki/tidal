#ifndef TIDAL_ODR_TWICE_TEMPLATE_H
#define TIDAL_ODR_TWICE_TEMPLATE_H
template <class T> T twice(T v) { return v + v; }
template <class T> int& tally() {
  static int count = 0;
  return count;
}
using IntFunction = int (*)(int);
IntFunction twice_elsewhere();
int* tally_elsewhere();
#endif
