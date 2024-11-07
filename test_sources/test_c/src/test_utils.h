// test_utils.h
#ifndef TEST_UTILS_H
#define TEST_UTILS_H

// memory op
void *allocate_memory(size_t size);
void *reallocate_memory(void *ptr, size_t new_size);
void free_memory(void *ptr);

// string op
char *string_duplicate(const char *str);
int string_compare(const char *str1, const char *str2);
char *string_concatenate(const char *str1, const char *str2);

// sturct op
void swap_generic(void *a, void *b, size_t size);
void reverse_array(void *array, size_t count, size_t elem_size);
void *binary_search(const void *key, const void *base,
                    size_t count, size_t elem_size,
                    int (*compare)(const void *, const void *));

// test macro
#define TEST_ASSERT(condition)               /* ... */
#define TEST_EQUALS(expected, actual)        /* ... */
#define TEST_STRING_EQUALS(expected, actual) /* ... */

#endif // TEST_UTILS_H