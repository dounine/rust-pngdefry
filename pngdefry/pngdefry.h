#ifndef _PNGDEFRY_H

#ifdef __cplusplus
extern "C" {
#endif

void restore_png(
        const char *filePath,
        char *error
);

int is_iphone_png(const char *filePath, char *error);

#ifdef __cplusplus
}
#endif

#endif