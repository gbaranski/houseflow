#include <Arduino.h>

String formatPayloadToString(uint8_t payload[], size_t length)
{
    String wholeString;
    for (unsigned int i = 0; i < length; i++)
    {
        wholeString += char(payload[i]);
    }
    return wholeString;
}