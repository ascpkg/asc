#ifndef AST_LIB
#define ABT_LIB

#ifdef __cplusplus
extern "C" {
#endif

int scan_necessary_sources(const char *entry_point_file, const char *source_dir, const char *target_dir, char *result_buf, int result_len);

#ifdef __cplusplus
}
#endif

#endif