#ifndef TIDAL_ODR_TAG_TEMPLATE_H
#define TIDAL_ODR_TAG_TEMPLATE_H
template <class T> struct Tag { static int count; };
template <class T> int Tag<T>::count = 0;
template <> struct Tag<char> { static int count; };
int Tag<char>::count = 0;
int* tag_elsewhere();
#endif
