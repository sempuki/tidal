#ifndef TIDAL_ODR_BOX_TEMPLATE_H
#define TIDAL_ODR_BOX_TEMPLATE_H
template <class T> struct Box {
  T value;
  T get() const { return value; }
  T doubled() const;
  static int made;
};
template <class T> T Box<T>::doubled() const { return value + value; }
template <class T> int Box<T>::made = 0;
int* made_elsewhere();
int doubled_elsewhere(Box<int> box);
#endif
