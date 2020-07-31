#include <Arduino.h>

boolean checkArrays(uint8_t arrayA[], uint8_t arrayB[], size_t numItemsArrayA, size_t numItemsArrayB, boolean preferSecondArray)
{
    boolean same = true;
    size_t numItems = preferSecondArray ? numItemsArrayB : numItemsArrayA > numItemsArrayB ? numItemsArrayA : numItemsArrayB;
    for (int i = 0; i < numItems && same; i++)
    {
        same = arrayA[i] == arrayB[i];
    }
    return same;
}

String formatPayloadToString(uint8_t payload[], size_t length)
{
    String wholeString;
    for (int i = 0; i < length; i++)
    {
        wholeString += char(payload[i]);
    }
    return wholeString;
}