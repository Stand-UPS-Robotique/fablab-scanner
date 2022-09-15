#include <Arduino.h>

#include <SPI.h>
#include <MFRC522.h>
#include "main.h"

MFRC522 reader;
State state;

void setup() {
  SPI.begin();
  reader.PCD_Init(RST_PIN);
  pinMode(BUZZER_PIN, OUTPUT);

  state = { "", millis() };

  Serial.begin(9600);
}

void loop() {
  if(reader.PICC_IsNewCardPresent() && reader.PICC_ReadCardSerial()) {
    String read_uid = get_uid(reader);
    
    if(!state.uid.equals(read_uid) || (state.uid.equals(read_uid) && (millis() - state.capture_time >= UID_RESET_DELAY))) {
      // Update state
      state = { read_uid, millis() };

      // Send the card UID over serial 
      Serial.println(state.uid);

      // Ring the buzzer to indicate found card
      digitalWrite(BUZZER_PIN, HIGH);
      delay(BUZZER_DELAY);
      digitalWrite(BUZZER_PIN, LOW);
    }
  }
}

// Converts byte array to a hex string
String get_uid(MFRC522 reader) {
  String tmp = "";
  for (byte i = 0; i < reader.uid.size; i++) {
    tmp.concat(reader.uid.uidByte[i] < 0x10 ? " 0" : " ");
    tmp.concat(String(reader.uid.uidByte[i], HEX));
  }

  return tmp;
}