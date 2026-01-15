import { useState } from "react";
import { Cpu, HardDrive, Clock, Copy, Check, Activity } from "lucide-react";

function App() {
  const [tunnelActive, setTunnelActive] = useState(false);
  const [tunnelUrl, setTunnelUrl] = useState("");
  const [copied, setCopied] = useState(false);
  const [provider, setProvider] = useState("cloudflare");
  const [token, setToken] = useState("");

  const handleStartTunnel = () => {
    if (!token.trim()) return;
    setTunnelActive(true);
    setTunnelUrl(`https://example-${provider}.zexio.dev`);
  };

  const handleStopTunnel = () => {
    setTunnelActive(false);
    setTunnelUrl("");
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(tunnelUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-zinc-950 text-white">
      {/* Header */}
      <div className="bg-zinc-900 border-b border-zinc-800 px-4 py-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <img src="/logo.png" alt="Zexio" className="w-8 h-8" />
            <div>
              <h1 className="font-semibold text-base">Zexio Agent</h1>
              <p className="text-xs text-zinc-500">Local Dashboard</p>
            </div>
          </div>
          <div className="flex items-center gap-1.5 px-2.5 py-1 bg-emerald-500/10 border border-emerald-500/20 rounded-full">
            <div className="w-1.5 h-1.5 rounded-full bg-emerald-500" />
            <span className="text-xs font-medium text-emerald-500">Online</span>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="p-4 space-y-4">
        {/* Stats Grid */}
        <div className="grid grid-cols-3 gap-3">
          {/* CPU Card */}
          <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-3">
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs text-zinc-500">CPU</span>
              <Cpu className="w-4 h-4 text-emerald-500" />
            </div>
            <p className="text-xl font-bold text-emerald-500">12%</p>
          </div>

          {/* Memory Card */}
          <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-3">
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs text-zinc-500">Memory</span>
              <HardDrive className="w-4 h-4 text-zinc-400" />
            </div>
            <p className="text-xl font-bold text-zinc-100">2.4 GB</p>
          </div>

          {/* Uptime Card */}
          <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-3">
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs text-zinc-500">Uptime</span>
              <Clock className="w-4 h-4 text-emerald-500" />
            </div>
            <p className="text-xl font-bold text-emerald-500">3h 24m</p>
          </div>
        </div>

        {/* Tunnel Card */}
        <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <Activity className={`w-4 h-4 ${tunnelActive ? 'text-emerald-500' : 'text-zinc-500'}`} />
              <h2 className="font-semibold text-sm">Tunnel</h2>
            </div>
            <div className={`flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium ${tunnelActive
                ? 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/20'
                : 'bg-zinc-800 text-zinc-500'
              }`}>
              <div className={`w-1.5 h-1.5 rounded-full ${tunnelActive ? 'bg-emerald-500' : 'bg-zinc-600'}`} />
              {tunnelActive ? "Active" : "Inactive"}
            </div>
          </div>

          <p className="text-xs text-zinc-500 mb-3">
            {tunnelActive ? "Tunnel is running" : "Start a secure tunnel"}
          </p>

          {!tunnelActive && (
            <div className="space-y-3 mb-3">
              <div>
                <label className="block text-xs text-zinc-400 mb-1.5">Provider</label>
                <select
                  value={provider}
                  onChange={(e) => setProvider(e.target.value)}
                  className="w-full px-3 py-2 bg-zinc-950 border border-zinc-800 rounded-lg text-sm text-white focus:outline-none focus:border-blue-500"
                >
                  <option value="cloudflare">Cloudflare Tunnel</option>
                  <option value="pangolin">Pangolin Tunnel</option>
                </select>
              </div>

              <div>
                <label className="block text-xs text-zinc-400 mb-1.5">Auth Token</label>
                <input
                  type="password"
                  value={token}
                  onChange={(e) => setToken(e.target.value)}
                  placeholder="Enter your tunnel token..."
                  className="w-full px-3 py-2 bg-zinc-950 border border-zinc-800 rounded-lg text-sm text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500"
                />
              </div>
            </div>
          )}

          <button
            onClick={tunnelActive ? handleStopTunnel : handleStartTunnel}
            className={`w-full py-2 px-4 rounded-lg text-sm font-medium transition-colors ${tunnelActive
                ? 'bg-red-600 hover:bg-red-500 text-white'
                : 'bg-blue-600 hover:bg-blue-500 text-white'
              }`}
          >
            {tunnelActive ? 'Stop Tunnel' : 'Start Tunnel'}
          </button>
        </div>

        {/* Public URL Card */}
        {tunnelActive && tunnelUrl && (
          <div className="bg-zinc-900 border border-emerald-500/20 rounded-lg p-4">
            <h3 className="text-sm font-semibold mb-2">Public URL</h3>
            <div className="flex items-center gap-2 p-2.5 bg-zinc-950 border border-zinc-800 rounded-lg">
              <code className="text-xs text-blue-400 font-mono flex-1 truncate">
                {tunnelUrl}
              </code>
              <button
                onClick={handleCopy}
                className={`p-1.5 rounded transition-colors ${copied
                    ? 'bg-emerald-500/10 text-emerald-500'
                    : 'bg-blue-600 hover:bg-blue-500 text-white'
                  }`}
              >
                {copied ? <Check className="w-3.5 h-3.5" /> : <Copy className="w-3.5 h-3.5" />}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
