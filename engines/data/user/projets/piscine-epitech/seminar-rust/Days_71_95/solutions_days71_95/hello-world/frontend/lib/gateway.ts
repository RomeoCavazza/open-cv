//! Client WebSocket Gateway
//! Gère la connexion, reconnexion, heartbeat, et dispatch des événements

import { Message } from "./api-server";

const HEARTBEAT_INTERVAL = 30000; // 30s
const RECONNECT_DELAY_INITIAL = 1000; // 1s
const RECONNECT_DELAY_MAX = 30000; // 30s
const RECONNECT_MAX_ATTEMPTS = 10;

export type ClientEvent =
  | { op: "IDENTIFY"; d: { token: string } }
  | { op: "SEND_MESSAGE"; d: { channel_id: string; content: string } }
  | { op: "TYPING_START"; d: { channel_id: string } }
  | { op: "TYPING_STOP"; d: { channel_id: string } }
  | { op: "HEARTBEAT"; d: { seq?: number } }
  | { op: "SUBSCRIBE"; d: { channel_id: string } }
  | { op: "UNSUBSCRIBE"; d: { channel_id: string } }
  | { op: "PRESENCE_UPDATE"; d: { status: string } };

export type ServerEvent =
  | { op: "HELLO"; d: { heartbeat_interval: number } }
  | { op: "READY"; d: { user_id: string; username: string } }
  | { op: "ERROR"; d: { code: string; message: string } }
  | { op: "MESSAGE_CREATE"; d: Message }
  | { op: "MESSAGE_UPDATE"; d: { id: string; channel_id: string; content: string; edited_at: string } }
  | { op: "MESSAGE_DELETE"; d: { id: string; channel_id: string } }
  | { op: "TYPING_START"; d: { channel_id: string; user_id: string; username: string } }
  | { op: "TYPING_STOP"; d: { channel_id: string; user_id: string } }
  | { op: "HEARTBEAT_ACK"; d: { seq?: number } }
  | { op: "SUBSCRIBED"; d: { channel_id: string } }
  | { op: "UNSUBSCRIBED"; d: { channel_id: string } }
  | { op: "PRESENCE_UPDATE"; d: { user_id: string; status: string } };

type EventHandler = (event: ServerEvent) => void;

export class Gateway {
  private ws: WebSocket | null = null;
  private url: string;
  private token: string | null = null;
  private heartbeatInterval: number | null = null;
  private reconnectTimeout: number | null = null;
  private reconnectAttempts = 0;
  private reconnectDelay = RECONNECT_DELAY_INITIAL;
  private heartbeatSeq = 0;
  private eventHandlers: Set<EventHandler> = new Set();
  private state: "disconnected" | "connecting" | "connected" | "authenticated" = "disconnected";
  private heartbeatIntervalMs = HEARTBEAT_INTERVAL;

  constructor(url: string) {
    this.url = url;
  }

  connect(token: string) {
    if (this.state === "connected" || this.state === "authenticated") {
      this.disconnect();
    }

    this.token = token;
    this.state = "connecting";
    this.reconnectAttempts = 0;
    this.reconnectDelay = RECONNECT_DELAY_INITIAL;

    this.doConnect();
  }

  private doConnect() {
    try {
      // Convertir http:// en ws:// ou https:// en wss://
      const wsUrl = this.url.replace(/^http/, "ws") + "/ws";
      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => {
        console.log("[Gateway] Connected");
        this.state = "connected";
        this.reconnectAttempts = 0;
        this.reconnectDelay = RECONNECT_DELAY_INITIAL;

        // Envoyer IDENTIFY après un court délai pour s'assurer que le WebSocket est vraiment prêt
        if (this.token) {
          const token = this.token; // Capture pour TypeScript
          setTimeout(() => {
            console.log("[Gateway] Sending IDENTIFY, readyState:", this.ws?.readyState);
            this.send({ op: "IDENTIFY", d: { token } });
          }, 100);
        }
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data) as ServerEvent;
          console.log("[Gateway] Received:", data.op);
          this.handleEvent(data);
        } catch (e) {
          console.error("[Gateway] Failed to parse message:", e);
        }
      };

      this.ws.onerror = (error) => {
        console.error("[Gateway] WebSocket error:", error);
      };

      this.ws.onclose = (event) => {
        console.log("[Gateway] Disconnected, code:", event.code, "reason:", event.reason);
        this.state = "disconnected";
        this.stopHeartbeat();

        // Tentative de reconnexion automatique
        if (this.token && this.reconnectAttempts < RECONNECT_MAX_ATTEMPTS) {
          this.scheduleReconnect();
        }
      };
    } catch (e) {
      console.error("[Gateway] Failed to connect:", e);
      this.state = "disconnected";
      if (this.token && this.reconnectAttempts < RECONNECT_MAX_ATTEMPTS) {
        this.scheduleReconnect();
      }
    }
  }

  private scheduleReconnect() {
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
    }

    this.reconnectAttempts++;
    const delay = Math.min(
      this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1),
      RECONNECT_DELAY_MAX
    );

    console.log(`[Gateway] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);

    this.reconnectTimeout = window.setTimeout(() => {
      this.doConnect();
    }, delay);
  }

  private handleEvent(event: ServerEvent) {
    switch (event.op) {
      case "HELLO":
        this.heartbeatIntervalMs = event.d.heartbeat_interval;
        this.startHeartbeat();
        break;

      case "READY":
        this.state = "authenticated";
        console.log("[Gateway] Authenticated as", event.d.username);
        break;

      case "ERROR":
        console.error("[Gateway] Server error:", event.d.code, event.d.message);
        // Si erreur d'auth, ne pas reconnecter
        if (event.d.code === "INVALID_TOKEN" || event.d.code === "USER_NOT_FOUND") {
          this.disconnect();
          return;
        }
        break;

      case "HEARTBEAT_ACK":
        // Heartbeat reçu, tout va bien
        break;
    }

    // Notifier tous les handlers
    this.eventHandlers.forEach((handler) => {
      try {
        handler(event);
      } catch (e) {
        console.error("[Gateway] Handler error:", e);
      }
    });
  }

  private startHeartbeat() {
    this.stopHeartbeat();

    this.heartbeatInterval = window.setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.heartbeatSeq++;
        this.send({ op: "HEARTBEAT", d: { seq: this.heartbeatSeq } });
      }
    }, this.heartbeatIntervalMs);
  }

  private stopHeartbeat() {
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
  }

  send(event: ClientEvent) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(event));
    } else {
      console.warn("[Gateway] Cannot send, WebSocket not open");
    }
  }

  onEvent(handler: EventHandler) {
    this.eventHandlers.add(handler);
    return () => {
      this.eventHandlers.delete(handler);
    };
  }

  subscribe(channelId: string) {
    this.send({ op: "SUBSCRIBE", d: { channel_id: channelId } });
  }

  unsubscribe(channelId: string) {
    this.send({ op: "UNSUBSCRIBE", d: { channel_id: channelId } });
  }

  sendMessage(channelId: string, content: string) {
    this.send({ op: "SEND_MESSAGE", d: { channel_id: channelId, content } });
  }

  typingStart(channelId: string) {
    this.send({ op: "TYPING_START", d: { channel_id: channelId } });
  }

  typingStop(channelId: string) {
    this.send({ op: "TYPING_STOP", d: { channel_id: channelId } });
  }

  updatePresence(status: "online" | "offline" | "dnd" | "invisible") {
    this.send({ op: "PRESENCE_UPDATE", d: { status } });
  }

  disconnect() {
    this.stopHeartbeat();
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.state = "disconnected";
    this.token = null;
  }

  isConnected(): boolean {
    return this.state === "authenticated" && this.ws?.readyState === WebSocket.OPEN;
  }
}

// Instance singleton
let gatewayInstance: Gateway | null = null;

export function getGateway(apiUrl: string): Gateway {
  if (!gatewayInstance) {
    gatewayInstance = new Gateway(apiUrl);
  }
  return gatewayInstance;
}

