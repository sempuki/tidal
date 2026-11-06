// Injected by the X23 scan. A deleted aligned non-allocating placement form:
// a placement new of an over-aligned type with a void* argument selects it and fails.
namespace std { using size_t = decltype(sizeof 0); enum class align_val_t : size_t; }
void* operator new(std::size_t, std::align_val_t, void*) = delete;
void* operator new[](std::size_t, std::align_val_t, void*) = delete;
