#include <stdio.h>
#include <stdlib.h>
#include <graphviz/gvc.h>

char *dot_to_svg(char *dot, int len) {
    FILE *infile = fmemopen(dot, len, "r");
    if (infile == NULL) {
        perror("fmemopen");
        return NULL;
    }

    char *buffer = 0;
    size_t bufflen = 0;
    FILE *outfile = open_memstream(&buffer, &bufflen);

    GVC_t *gvc = gvContext();
    Agraph_t *g = agread(infile, 0);

    gvLayout(gvc, g, "dot");
    gvRender(gvc, g, "svg", outfile);

    gvFreeLayout(gvc, g);
    agclose(g);
    gvFreeContext(gvc);
    fclose(infile);
    fclose(outfile);

    return buffer;
}

void c_free(char *buffer) {
    free(buffer);
}
