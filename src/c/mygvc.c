#include <stdio.h>
#include <stdlib.h>
#include <graphviz/gvc.h>

char *dot_to_svg(char *dot, int len) {
    char *buffer = NULL;
    size_t bufflen = 0;

    GVC_t *gvc = gvContext();
    if (gvc == NULL) {
        fprintf(stderr, "%s\n", "gvContext failed");
    } else {
        FILE *infile = fmemopen(dot, len, "r");
        if (infile == NULL) {
            perror("fmemopen");
        } else {
            Agraph_t *g = agread(infile, 0);
            if (g == NULL) {
                fprintf(stderr, "%s\n", "agread failed.");
            } else {
                if (gvLayout(gvc, g, "dot") != 0) {
                    fprintf(stderr, "%s\n", "gvLayout failed.");
                } else {
                    FILE *outfile = open_memstream(&buffer, &bufflen);
                    if (outfile == NULL) {
                        perror("open_memstream");
                    } else {
                        if (gvRender(gvc, g, "svg", outfile) != 0) {
                            fprintf(stderr, "%s\n", "gvRender failed.");
                            free(buffer);
                            buffer = NULL;
                        }
                        gvFreeContext(gvc);
                    }
                    fclose(outfile);
                }
                gvFreeLayout(gvc, g);
            }
            agclose(g);
        }
        fclose(infile);
    }

    return buffer;
}

void c_free(char *buffer) {
    free(buffer);
}
