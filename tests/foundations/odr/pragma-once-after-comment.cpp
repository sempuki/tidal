// tidal-rules: INC-5
// tidal-expect: run
#include "once-after-comment.h"
#include "once-after-comment.h"
int main() { OnceCommented o{3}; return o.v - 3; }
