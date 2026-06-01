import { useState, useEffect, useRef } from 'react';
import './index.css';

interface UdpCanFrame {
  id: number;
  data: number[];
}

interface LogEntry {
  timestamp: string;
  frame: UdpCanFrame;
}

function App() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [connected, setConnected] = useState(false);
  const ws = useRef<WebSocket | null>(null);
  const logsEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    connectWebSocket();
    return () => {
      ws.current?.close();
    };
  }, []);

  useEffect(() => {
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  const connectWebSocket = () => {
    ws.current = new WebSocket('ws://localhost:3000/ws');

    ws.current.onopen = () => {
      setConnected(true);
    };

    ws.current.onclose = () => {
      setConnected(false);
      setTimeout(connectWebSocket, 3000);
    };

    ws.current.onmessage = (event) => {
      try {
        const frame: UdpCanFrame = JSON.parse(event.data);
        const entry: LogEntry = {
          timestamp: new Date().toISOString().split('T')[1].slice(0, -1),
          frame
        };
        setLogs(prev => [...prev.slice(-100), entry]); // Keep last 100
      } catch (e) {
        console.error('Failed to parse frame', e);
      }
    };
  };

  const sendCommand = (id: number, data: number[]) => {
    if (ws.current && ws.current.readyState === WebSocket.OPEN) {
      const frame: UdpCanFrame = { id, data };
      ws.current.send(JSON.stringify(frame));
    }
  };

  return (
    <div className="dashboard">
      <header className="header">
        <h1>SpaceCAN Mission Control</h1>
        <div className="status-indicator">
          <span>{connected ? 'WS CONNECTED' : 'WS DISCONNECTED'}</span>
          <div className={`dot ${connected ? 'connected' : ''}`}></div>
        </div>
      </header>

      <aside className="glass-panel">
        <h2>Telecommands</h2>
        
        <button className="btn" onClick={() => sendCommand(0x080, [])}>
          <span>SYNC Frame</span>
          <span>ST09</span>
        </button>
        
        <button className="btn" onClick={() => sendCommand(0x102, [1, 0])}>
          <span>Verify Request</span>
          <span>ST01</span>
        </button>
        
        <button className="btn" onClick={() => sendCommand(0x301, [1])}>
          <span>Housekeeping Report</span>
          <span>ST03</span>
        </button>
        
        <button className="btn" onClick={() => sendCommand(0x801, [])}>
          <span>Function Management</span>
          <span>ST08</span>
        </button>

        <button className="btn" onClick={() => sendCommand(0x1101, [1])}>
          <span>Connection Test</span>
          <span>ST17</span>
        </button>

        <button className="btn" onClick={() => sendCommand(0x1401, [1])}>
          <span>Report Parameters</span>
          <span>ST20</span>
        </button>
      </aside>

      <main className="glass-panel">
        <h2>Live Telemetry Stream</h2>
        <div className="log-container">
          {logs.length === 0 ? (
            <div style={{ color: 'var(--text-secondary)', textAlign: 'center', marginTop: '2rem' }}>
              Waiting for telemetry data on UDP Multicast 224.0.0.123:5000...
            </div>
          ) : (
            logs.map((log, i) => (
              <div key={i} className="log-entry">
                <span className="log-time">[{log.timestamp}]</span>
                <span className="log-id">ID: 0x{log.frame.id.toString(16).toUpperCase().padStart(3, '0')}</span>
                <span className="log-data">
                  DATA: [{log.frame.data.map(b => '0x' + b.toString(16).padStart(2, '0')).join(', ')}]
                </span>
              </div>
            ))
          )}
          <div ref={logsEndRef} />
        </div>
      </main>
    </div>
  );
}

export default App;
