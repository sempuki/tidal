#include "box-template.h"
int* made_elsewhere() { return &Box<int>::made; }
int doubled_elsewhere(Box<int> box) { return box.doubled(); }
