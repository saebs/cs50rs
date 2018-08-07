// Sabelo Ntabeni
// CS50 2018
// Instructor: David J Malan
// pset1 credit
#include <stdio.h>
#include <cs50.h>
#include <stdlib.h>

int main(void)
{
    // Credit card number input
    long long  card_number;
    do
    {
        card_number = get_long_long("Number:");
    }
    while (card_number < 0);

    long long card_cpy1, card_cpy2;
    card_cpy1 = card_number;
    card_cpy2 = card_number;

    int number_len = 0;
    while (card_cpy1 > 0)
    {
        card_cpy1 = card_cpy1 / 10;
        number_len++;
    };

    // Indexable number
    int digit_set[number_len];
    int i;
    for (i = number_len - 1; i >= 0; i--)
    {
        digit_set[i] = card_cpy2 % 10;
        card_cpy2 = card_cpy2 / 10;
    };
    int checksum = 0;
    int j;
    int cue = 0;
    int prod = 0;

    // Summing product digits and summing digits
    for (j = number_len - 1; j >= 0; j--)
    {
        // Action cue
        // Cue 1 : add product digit(s) in next cycle
        // Cue 0: add digit only in next cycle
        if (cue == 1)
        {
            prod = digit_set[j] * 2;
            // The proposition: for all integers 0 - 9,
            // the product of any two will have at most two digits.
            checksum += prod % 10;
            checksum += prod / 10;
            prod = 0;
            cue = 0;
        }
        else
        {
            checksum += digit_set[j];
            cue = 1;
        };
    };

    // Validate
    if (checksum % 10 != 0)
    {
        printf("INVALID\n");
        exit(0);
        return 0;
    };
    // Validate Company
    // AMERICAN EXPRESS  ID 34.. 37..  , NUMBERLENTGH = 15 digits
    // MASTERCARD ID 51..55 , LENGTH : 16 digits
    // VISA ID 4.. , LENGTH : 13 or 16
    if (digit_set[0] == 3 && (digit_set[1] == 4 || digit_set[1] == 7) && number_len == 15)
    {
        printf("AMEX\n");
        exit(0);
    }
    else if (digit_set[0] == 4 && (number_len == 13 || number_len == 16))
    {
        printf("VISA\n");
        exit(0);
    }
    else if (digit_set[0] == 5 && (digit_set[1] >= 1 && digit_set[1] <= 5) && number_len == 16)
    {
        printf("MASTERCARD\n");
        exit(0);
    }
    else
    {
        printf("INVALID\n");
    };
    return 0;
}
