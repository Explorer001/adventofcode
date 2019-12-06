#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

int get_input_array(char *filename, int **dest)
{

    FILE *fptr;
    if((fptr = fopen(filename, "r")) == NULL)
    {
        printf("failed to open file: %s\n", filename);
        return -1;
    }

    int32_t nmbr;
    size_t input_size = 0;

    /* get size of input array */
    while (fscanf(fptr, "%d,", &nmbr) != EOF)
    {
        input_size++;
    }

    /* rewind file for second iteration */
    rewind(fptr);

    /* malloc input array */
    int *input = malloc(sizeof(int) * input_size);

    if (input == NULL)
        return -1;

    /* fill input array */
    input_size = 0;
    while (fscanf(fptr, "%d,", &nmbr) != EOF)
    {
        input[input_size] = nmbr;
        input_size++;
    }

    *dest = input;

    fclose(fptr);

    return input_size;
}

int do_intcode(int *input, int len)
{
    int instruction_prt = 0, op1, op2, dst, opcode;
    bool terminated = false;

    while (!terminated)
    {

        opcode = input[instruction_prt];

        switch (opcode)
        {
            case 1: /* addition */
                op1 = input[instruction_prt+1];
                op2 = input[instruction_prt+2];
                dst = input[instruction_prt+3];
                input[dst] = input[op1] + input[op2];
                break;
            case 2: /* mult */
                op1 = input[instruction_prt+1];
                op2 = input[instruction_prt+2];
                dst = input[instruction_prt+3];
                input[dst] = input[op1] * input[op2];
                break;
            case 99: /* halt */
                terminated = true;
                break;
        }

        instruction_prt += 4;
    }
    return input[0];
}

int main(int argc, char *argv[])
{

    if (argc != 2)
    {
        printf("usage: ./day1 <inputfile>\n");
        return -1;
    }

    int *input;
    int len = get_input_array(argv[1], &input);
    if (len < 0)
        return -1;

    /* replacements for part 1 */
    input[1] = 12;
    input[2] = 2;

    int *input_state = malloc(sizeof(int) * len);
    if (input_state == NULL)
    {
        free(input);
        return -1;
    }
    memcpy(input_state, input, sizeof(int) * len);

    /* get intcode */
    int val = do_intcode(input, len);
    printf("intcode: %d\n", val);

    /* find verb and noun */
    int noun_verb = 0, noun, verb;
    int result = 19690720;
    val = 0;

    while ((val != result) && noun_verb < 10000)
    {
        memcpy(input, input_state, sizeof(int) * len);
        noun = (noun_verb / 100);
        verb = noun_verb - (noun * 100);
        input[1] = noun;
        input[2] = verb;
        val = do_intcode(input, len);
        noun_verb++;
    }
    printf("noun: %d, verb: %d\n", noun, verb);
    printf("%d\n", 100 * noun + verb);

    free(input);
    free(input_state);



    return 0;
}