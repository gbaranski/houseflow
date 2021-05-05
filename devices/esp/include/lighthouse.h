#include <WebSocketsClient.h>

class Lighthouse {
  public:
    void loop();
    Lighthouse() {
      Serial.println("Constructor called");
      ws_client = WebSocketsClient();
    }

  private:
    WebSocketsClient ws_client;
};
