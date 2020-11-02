#include <stdio.h>
#include <stdlib.h>

#define USAGE -1
#define NOT_FOUND -2

int main(int argc, char **argv)
{
    if (argc != 2)
    {
        printf("Usage : bf <filename>");
        return USAGE;
    }
    FILE *fp;
    if (fp = fopen(argv[1], "r"))
    {
        fseek(fp, 0L, SEEK_END);
        const long size = ftell(fp) + 1;
        rewind(fp);

        char *buf = malloc(sizeof(char) * size);

        fgets(buf, size, fp);

        printf("%s", buf);
    }
    else
    {
        printf("File not found");
        return NOT_FOUND;
    }

    return 0;
}