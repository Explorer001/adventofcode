#include <stdio.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

#define INT_ARRAY_SIZE 16

bool is_valid_passwd(int passwd, bool grp)
{
    /* convert int to int array */
    int passwd_array[INT_ARRAY_SIZE];
    size_t index = 0;

    while (passwd != 0)
    {
        passwd_array[index++] = passwd % 10;
        passwd /= 10;
    }

    /* passwd need to be six digit number */
    if (index != 6)
        return false;

    int last_adjacent = -1;
    int adj_count = 0, min_adj_count = 1000;

    /* sanity check */
    for (int i = index - 1; i > 0; i--)
    {
        if (passwd_array[i] > passwd_array[i-1])
            return false;
        else if (passwd_array[i] == passwd_array[i-1])
        {
            if (passwd_array[i] == last_adjacent)
            {
                adj_count++;
            }
            else
            {
                /* check minimum number of adjacents */
                if ((adj_count < min_adj_count) && (last_adjacent != -1))
                {
                    min_adj_count = adj_count;
                }
                last_adjacent = passwd_array[i];
                adj_count = 1;
            } 
        }
    }

    if (adj_count < min_adj_count)
        min_adj_count = adj_count;

    return (grp)? (min_adj_count == 1) : (min_adj_count > 0);
}

int main(int argc, char *argv[])
{
    /* parse range */
    if (argc != 2)
    {
        printf("usage: ./day4 <lower>-<upper>\n");
        return -1;
    }

    /* extract lower and upper bound from arg */
    char *saveptr;

    char *lower_str = strtok_r(argv[1], "-", &saveptr);
    char *upper_str = strtok_r(NULL, "-", &saveptr);
    if ((lower_str == NULL) || (upper_str == NULL))
    {
        printf("invalid range: %s\n", argv[1]);
        return -1;
    }

    /* convert ranges to int */
    int lower, upper;

    lower = (int) strtol(lower_str, NULL, 10);
    upper = (int) strtol(upper_str, NULL, 10);
    if (lower == 0)
    {
        printf("invalid lower range: %s\n", lower_str);
        return -1;
    }

    if (upper == 0)
    {
        printf("invalid upper range: %s\n", upper_str);
        return -1;
    }

    /* count valid passwds int range */
    int valid_passwds = 0;
    int valid_passwds_grp = 0;

    for (int i = lower; i <=upper; i++)
    {
        if (is_valid_passwd(i, false))
            valid_passwds++;
        if (is_valid_passwd(i, true))
            valid_passwds_grp++;
    }

    printf("valid passwords in range %d-%d: %d\n", lower, upper, valid_passwds);
    printf("valid passwords (grp) in range %d-%d: %d\n", lower, upper, valid_passwds_grp);

    return 0;
}