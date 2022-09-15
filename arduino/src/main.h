#ifndef H_MAIN
#define H_MAIN

#include <MFRC522.h>

// Constants
#define RST_PIN 9
#define BUZZER_PIN 2

#define BUZZER_DELAY    200
#define UID_RESET_DELAY 1800

// Functions
String get_uid(MFRC522 reader);

// Structs
typedef struct {
    String uid;
    unsigned long capture_time;
} State;

#endif