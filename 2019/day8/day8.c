#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>

#define PIXEL_BLACK 0
#define PIXEL_WHITE 1
#define PIXEL_TRANSPARENT 2

int get_pxl_map(char *filename, uint8_t **dest)
{
    FILE *fptr;

    /* try to open input file */
    if ((fptr = fopen(filename, "r")) == NULL)
    {
        printf("No such file or directory: %s\n", filename);
        return -1;
    }

    size_t nbytes;

    /* get size of file */
    fseek(fptr, 0L, SEEK_END);
    nbytes = (size_t) ftell(fptr);
    rewind(fptr);

    /* read whole file to array */
    char buffer[nbytes];
    if (fread(buffer, nbytes, 1, fptr) != 1)
    {
        printf("Failed to read file content\n");
        return -1;
    }

    fclose(fptr);

    /* allocate int array ; -1 to exclude trailing \n of input */
    uint8_t *int_array = malloc(nbytes * sizeof(uint8_t) - 1);
    
    if (int_array == NULL)
    {
        printf("Failed to allocate input array of size %lu\n", nbytes);
    }

    /* translate input chars to ints ; -1 to exclude trailing \n of input */
    for (size_t i = 0; i < (nbytes - 1); i++)
    {
        int_array[i] = (uint8_t) (buffer[i] - '0');
    }

    *dest = int_array;

    return nbytes - 1;
}

int layer_get_digit_count(uint8_t *src, size_t width, size_t height, int layer, uint8_t digit)
{
    size_t index;
    size_t layer_end = width * height * (layer + 1);
    int count = 0;

    for (index = width * height * layer; index < layer_end; index ++)
    {
        if (src[index] == digit)
            count++;
    }
    return count;
}

int find_layer_with_fewest(uint8_t *src, size_t len, size_t width, size_t height, uint8_t fewest)
{
    int global_min = 1000000, local_min = 0;
    int layer = 0;

    size_t layer_count = len / (width * height);

    for (size_t i = 0; i < layer_count; i++)
    {
        local_min = layer_get_digit_count(src, width, height, i, fewest);
        if (local_min < global_min)
        {
            global_min = local_min;
            layer = (int) i;
        }
    }

    return layer + 1;

}

int generate_image(uint8_t *src, size_t nbytes, size_t width, size_t height, uint8_t **dst)
{
    /* allocate memory for image */
    size_t layer_size = width * height;
    uint8_t *img = malloc(layer_size * sizeof(uint8_t));

    if (img == NULL)
    {
        printf("Failed to allocate image memory\n");
        return -1;
    }

    /* initailize image with transpatent pxls */
    memset(img, PIXEL_TRANSPARENT, layer_size * sizeof(uint8_t));

    size_t img_pos = 0;

    /* iterate trhough image and fill pixels that are not transparent */
    for (size_t index = 0; index < nbytes; index++)
    {
        img_pos = index % layer_size;

        /* fill transparent pixel */
        if (img[img_pos] == PIXEL_TRANSPARENT)
        {
            img[img_pos] = src[index];
        }

    }

    *dst = img;

    return layer_size;
}

void print_img(uint8_t *img, size_t len, size_t width, size_t height)
{
    for (size_t index = 0; index < len; index++)
    {
        if (((index % width) == 0) && (index != 0))
            printf("\n");
        if (img[index] == PIXEL_BLACK)
            printf(".");
        if (img[index] == PIXEL_WHITE)
            printf("#");
    }
}

int main(int argc, char *argv[])
{

    if (argc != 4)
    {
        printf("Usage: %s <filename> <width>x<height> <fewest>:<dig1>:<dig2>\n", argv[0]);
        return -1;
    }

    /* parse width and height */
    char *saveptr;

    char *width_str = strtok_r(argv[2], "x", &saveptr);
    char *height_str = strtok_r(NULL, "x", &saveptr);

    if ((width_str == NULL) || (height_str == NULL))
    {
        printf("Illegal dimensions: %s\n", argv[2]);
        return -1;
    }

    size_t width, height;

    width = (size_t) strtol(width_str, NULL, 10);
    height = (size_t) strtol(height_str, NULL, 10);

    if (width == 0)
    {
        printf("Invalid width: %s\n", width_str);
        return -1;
    }

    if (height == 0)
    {
        printf("Invalid heigth: %s\n", height_str);
        return -1;
    }

    /* parse info for part 1 */
    char *fewest_str = strtok_r(argv[3], ":", &saveptr);
    char *pos1_str = strtok_r(NULL, ":", &saveptr);
    char *pos2_str = strtok_r(NULL, ":", &saveptr);

    if ((fewest_str == NULL) || (pos1_str == NULL) || (pos2_str == NULL))
    {
        printf("Illegal arg: %s\n", argv[3]);
        return -1;
    }

    int fewest, pos1, pos2;
    fewest = (int) strtol(fewest_str, NULL, 10);
    pos1 = (int) strtol(pos1_str, NULL, 10);
    pos2 = (int) strtol(pos2_str, NULL, 10);

    /* get input array */
    uint8_t *input;
    int rv = get_pxl_map(argv[1], &input);

    if (rv < 0)
    {
        return -1;
    }

    /* get min layer */
    int fewest_layer = find_layer_with_fewest(input, rv, width, height, fewest);
    printf("Layer with fewest %d: %d\n", fewest, fewest_layer);
    int checksum = layer_get_digit_count(input, width, height, fewest_layer - 1, pos1) *
                   layer_get_digit_count(input, width, height, fewest_layer - 1, pos2);
    printf("Layer sum: %d\n", checksum);

    /* generate layered image */
    uint8_t *img;
    rv = generate_image(input, rv, width, height, &img);

    /* display image */
    print_img(img, rv, width, height);

    free(input);
    free(img);

    return 0;
}