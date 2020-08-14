#include <Arduino.h>

#include <NTPClient.h>
#include <WiFiUdp.h>

WiFiUDP ntpUDP;
NTPClient timeClient(ntpUDP, "0.pl.pool.ntp.org", 7200, 60000);

String alarmTime = "07:45"; // default alarm time
bool alarmState = false;

void setupNtp()
{
    timeClient.begin();
}

void updateTime()
{
    timeClient.update();
}

String getCurrentTime()
{
    return timeClient.getFormattedTime();
}

String parseTimeToHour(String time)
{
    time.remove(2);
    return time;
}

String parseTimeToMinute(String time)
{
    time.remove(0, 3);
    return time;
}
String parseTimeToSeconds(String time)
{
    time.remove(0, 6);
    return time;
}

String formatDoubleDigit(String number)
{
    return number.toInt() < 10 ? "0" + number : number;
}

int parseTimeToTotalSeconds(String time)
{
    return parseTimeToHour(time).toInt() * 3600 + parseTimeToMinute(time).toInt() * 60 + parseTimeToSeconds(time).toInt();
}

void saveAlarmTime(String data)
{
    Serial.println("fulltime" + data);
    Serial.println("Hour:" + parseTimeToHour(data));
    Serial.println("Minute:" + parseTimeToMinute(data));
    alarmTime = data;
}

String getAlarmTime()
{
    return alarmTime;
}

String getFormattedRemainingTime()
{
    int totalRemainingSeconds = parseTimeToTotalSeconds(alarmTime + ":00") - parseTimeToTotalSeconds(timeClient.getFormattedTime());
    int remainingSeconds = totalRemainingSeconds % 60;
    int remainingMinutes = (totalRemainingSeconds / 60) % 60;
    int remainingHours = totalRemainingSeconds / 3600;

    if (remainingHours <= 0 && remainingMinutes <= 0 && remainingSeconds <= 0)
    {
        remainingHours = remainingHours + 23;
        remainingMinutes = remainingMinutes + 59;
        remainingSeconds = remainingSeconds + 59;
    }
    return formatDoubleDigit(String(remainingHours)) + ":" + formatDoubleDigit(String(remainingMinutes)) + ":" + formatDoubleDigit(String(remainingSeconds));
}
bool isNowAlarmTime()
{
    if (timeClient.getHours() == parseTimeToHour(alarmTime).toInt() && timeClient.getMinutes() == parseTimeToMinute(alarmTime).toInt())
    {
        return true;
    }
    return false;
}
String getAlarmState()
{
    if (alarmState)
    {
        return " ON";
    }
    else
    {
        return "OFF";
    }
}
bool getAlarmStateBoolean()
{
    return alarmState;
}
void setAlarmState(bool newAlarmState)
{
    alarmState = newAlarmState;
}
