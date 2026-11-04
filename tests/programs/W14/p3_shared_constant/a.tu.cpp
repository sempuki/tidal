#include "limits.h"

namespace limits {
const int* max_users_seen_by_a() { return &max_users; }
const int* page_size_seen_by_a() { return &page_size; }
}
