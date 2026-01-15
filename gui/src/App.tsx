import { useState } from "react";
import { TunnelControl } from "./components/TunnelControl";
import { StatusCard } from "./components/StatusCard";
import "./App.css";

function App() {
  const [tunnelActive, setTunnelActive] = useState(false);
  const [tunnelUrl, setTunnelUrl] = useState("");

  const handleStartTunnel = async (provider: string, token: string) => {
    // TODO: Call Tauri backend API
    console.log("Starting tunnel:", { provider, token });
    setTunnelActive(true);
    setTunnelUrl(`https://example-${provider}.tunnel.dev`);
  };

  const handleStopTunnel = async () => {
    // TODO: Call Tauri backend API
    console.log("Stopping tunnel");
    setTunnelActive(false);
    setTunnelUrl("");
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white font-sans">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 bg-gray-800 border-b border-gray-700 shadow-md">
        <div className="flex items-center gap-3">
          <img src="/logo.png" alt="Zexio Logo" className="w-10 h-10 object-contain" />
          <div>
            <h1 className="text-xl font-bold tracking-tight text-white">Zexio Agent</h1>
            <p className="text-xs text-gray-400">Local Dashboard</p>
          </div>
        </div>
        <div className="flex items-center gap-4">
          <div className="px-3 py-1 text-xs font-medium text-green-400 bg-green-400/10 rounded-full border border-green-400/20">
            ● Online
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="p-8 max-w-6xl mx-auto">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
          <StatusCard
            title="CPU Usage"
            value="12%"
            status="success"
            icon="⚡"
          />
          <StatusCard
            title="Memory"
            value="2.4 GB"
            status="neutral"
            icon="💾"
          />
          <StatusCard
            title="Uptime"
            value="3h 24m"
            status="success"
            icon="⏱️"
          />
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <TunnelControl
            isActive={tunnelActive}
            onStart={handleStartTunnel}
            onStop={handleStopTunnel}
          />

          {tunnelActive && tunnelUrl && (
            <div className="p-6 bg-gray-800 rounded-xl border border-green-500/30">
              <h3 className="text-sm font-medium text-gray-400 mb-3">Public URL</h3>
              <div className="flex items-center gap-2 p-3 bg-gray-900 rounded-lg border border-gray-700">
                <span className="text-sm text-blue-400 font-mono flex-1 truncate">{tunnelUrl}</span>
                <button
                  onClick={() => navigator.clipboard.writeText(tunnelUrl)}
                  className="px-3 py-1 bg-blue-600 hover:bg-blue-500 text-white text-xs font-semibold rounded transition-colors"
                >
                  Copy
                </button>
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
