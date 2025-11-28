struct ModelVertex {
    struct { float x, y, z; } pos, nrm;
    struct { float r, g, b, a; } col;
    struct { float u, v; } tex;
};

extern struct ModelVertex sphere_model[];
extern unsigned long sphere_model_len;
extern struct ModelVertex arrow_model[];
extern unsigned long arrow_model_len;
