/**
 * Resilient Server-Sent Events (SSE) Client with Exponential Backoff Reconnection.
 */

export interface SSEClientOptions {
  url: string;
  initialDelayMs?: number;
  maxDelayMs?: number;
  backoffMultiplier?: number;
  onMessage?: (event: MessageEvent) => void;
  onError?: (error: Event) => void;
  onOpen?: () => void;
}

export class ResilientSSEClient {
  private url: string;
  private initialDelayMs: number;
  private maxDelayMs: number;
  private backoffMultiplier: number;
  private currentDelayMs: number;
  private eventSource: EventSource | null = null;
  private isExplicitlyClosed = false;
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;

  public onMessage?: (event: MessageEvent) => void;
  public onError?: (error: Event) => void;
  public onOpen?: () => void;

  constructor(options: SSEClientOptions) {
    this.url = options.url;
    this.initialDelayMs = options.initialDelayMs ?? 1000;
    this.maxDelayMs = options.maxDelayMs ?? 30000;
    this.backoffMultiplier = options.backoffMultiplier ?? 2.0;
    this.currentDelayMs = this.initialDelayMs;
    this.onMessage = options.onMessage;
    this.onError = options.onError;
    this.onOpen = options.onOpen;
  }

  public connect(): void {
    if (this.isExplicitlyClosed) return;
    this.cleanup();

    try {
      this.eventSource = new EventSource(this.url);

      this.eventSource.onopen = () => {
        this.currentDelayMs = this.initialDelayMs;
        this.onOpen?.();
      };

      this.eventSource.onmessage = (event) => {
        this.onMessage?.(event);
      };

      this.eventSource.onerror = (error) => {
        this.onError?.(error);
        this.scheduleReconnect();
      };
    } catch {
      this.scheduleReconnect();
    }
  }

  private scheduleReconnect(): void {
    if (this.isExplicitlyClosed || this.reconnectTimer) return;

    this.cleanup();
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.currentDelayMs = Math.min(this.currentDelayMs * this.backoffMultiplier, this.maxDelayMs);
      this.connect();
    }, this.currentDelayMs);
  }

  private cleanup(): void {
    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = null;
    }
  }

  public close(): void {
    this.isExplicitlyClosed = true;
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    this.cleanup();
  }

  public getDelay(): number {
    return this.currentDelayMs;
  }
}
