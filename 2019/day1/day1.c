#include <stdio.h>
#include <stddef.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdbool.h>

int32_t calculate_fuel(char *filename, bool iterated)
{
    /* try to open input file */
    FILE *fptr = fopen(filename, "r");

    if (fptr == NULL)
    {
        printf("failed to open file with name: %s\n", filename);
        return -1;
    }

    /* iterate trhough input file and sum fuel values */

    int32_t fuel_sum = 0;
    int32_t fuel;

    while (fscanf(fptr, "%u\n", &fuel) != EOF)
    {
        if (iterated)
        {
            fuel = (fuel / 3) - 2;
            while (fuel > 0)
            {
                fuel_sum += fuel;
                fuel = (fuel / 3) - 2;
            }
        }
        else
        {
            fuel_sum += (fuel / 3) - 2;
        }
    }

    fclose(fptr);

    return fuel_sum;

}

int main(int argc, char *argv[])
{

    if (argc != 2)
    {
        printf("usage: ./day1 <inputfile>\n");
        return -1;
    }

    int32_t fuel = calculate_fuel(argv[1], false);
    printf("fuel: %d\n", fuel);

    fuel = calculate_fuel(argv[1], true);
    printf("fuel: %d\n", fuel);

    return 0;
}