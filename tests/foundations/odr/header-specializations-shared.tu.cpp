#include "kind-template.h"
int Kind<char>::count = 0;
int* pointer_count_elsewhere() { return &Kind<int*>::count; }
int* char_count_elsewhere() { return &Kind<char>::count; }
int* char_base_elsewhere() { return &base_of<char>; }
CharFunction width_elsewhere() { return &width<char>; }
