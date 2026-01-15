import { useState } from "react";
import { Settings, Activity } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./components/ui/select";

function App() {
  const [tunnelActive, setTunnelActive] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [provider, setProvider] = useState("cloudflare");
  const [token, setToken] = useState("");

  const handleToggle = () => {
    if (!tunnelActive && !token.trim()) {
      setShowSettings(true);
      return;
    }
    setTunnelActive(!tunnelActive);
  };

  return (
    <div className="h-screen bg-zinc-950 text-white flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-end px-6 py-4 border-b border-zinc-800">
        <button
          onClick={() => setShowSettings(!showSettings)}
          className="p-2 hover:bg-zinc-800 rounded-lg transition-colors"
        >
          <Settings className="w-5 h-5 text-zinc-400" />
        </button>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col items-center justify-center px-6">
        {!showSettings ? (
          <>
            {/* Logo */}
            <div className="mb-8">
              <img src="/logo.png" alt="Zexio" className="w-24 h-24 mx-auto mb-4" />
            </div>

            {/* Brand */}
            <div className="mb-12">
              <h2 className="text-5xl font-bold text-center mb-2 bg-gradient-to-r from-blue-500 to-cyan-500 bg-clip-text text-transparent">
                ZEXIO
              </h2>
              <p className="text-center text-sm text-zinc-500">Agent</p>
            </div>

            {/* Toggle Switch */}
            <button
              onClick={handleToggle}
              className="mb-8 relative"
            >
              <div className={`w-64 h-32 rounded-full transition-all duration-300 ${tunnelActive
                ? 'bg-gradient-to-r from-blue-500 to-cyan-500'
                : 'bg-zinc-800'
                }`}>
                <div className={`absolute top-2 w-28 h-28 bg-white rounded-full transition-all duration-300 ${tunnelActive ? 'right-2' : 'left-2'
                  }`} />
              </div>
            </button>

            {/* Status */}
            <div className="text-center mb-8">
              <h3 className="text-3xl font-bold mb-2">
                {tunnelActive ? 'Connected' : 'Disconnected'}
              </h3>
              <p className="text-zinc-500">
                {tunnelActive
                  ? 'Your tunnel is active and running'
                  : 'Click the switch to start tunnel'}
              </p>
            </div>

            {/* Stats (when connected) */}
            {tunnelActive && (
              <div className="w-full max-w-md space-y-3">
                <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-zinc-400">Provider</span>
                    <span className="text-sm font-medium capitalize">{provider}</span>
                  </div>
                </div>
                <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
                  <div className="flex items-center justify-between">
                    <span className="text-sm text-zinc-400">Public URL</span>
                    <code className="text-sm font-mono text-blue-400">
                      https://example.zexio.dev
                    </code>
                  </div>
                </div>
              </div>
            )}
          </>
        ) : (
          /* Settings Panel */
          <div className="w-full max-w-md space-y-4">
            <h2 className="text-2xl font-bold mb-6">Settings</h2>

            <div>
              <label className="block text-sm text-zinc-400 mb-2">
                Tunnel Provider
              </label>
              <Select value={provider} onValueChange={setProvider}>
                <SelectTrigger className="w-full bg-zinc-900 border-zinc-800 text-white">
                  <SelectValue placeholder="Select provider" />
                </SelectTrigger>
                <SelectContent className="bg-zinc-900 border-zinc-800">
                  <SelectItem value="cloudflare" className="text-white focus:bg-zinc-800 focus:text-white">
                    Cloudflare Tunnel
                  </SelectItem>
                  <SelectItem value="pangolin" className="text-white focus:bg-zinc-800 focus:text-white">
                    Pangolin Tunnel
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div>
              <label className="block text-sm text-zinc-400 mb-2">
                Authentication Token
              </label>
              <input
                type="password"
                value={token}
                onChange={(e) => setToken(e.target.value)}
                placeholder="Enter your tunnel token..."
                className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500"
              />
            </div>

            <button
              onClick={() => setShowSettings(false)}
              className="w-full py-3 bg-blue-600 hover:bg-blue-500 rounded-lg font-medium transition-colors"
            >
              Save Settings
            </button>
          </div>
        )}
      </div>

      {/* Footer Status */}
      <div className="px-6 py-3 border-t border-zinc-800">
        <div className="flex items-center justify-between text-sm">
          <div className="flex items-center gap-2">
            <Activity className={`w-4 h-4 ${tunnelActive ? 'text-emerald-500' : 'text-zinc-600'}`} />
            <span className="text-zinc-500">Agent Status</span>
          </div>
          <div className="flex items-center gap-2">
            <div className={`w-2 h-2 rounded-full ${tunnelActive ? 'bg-emerald-500' : 'bg-zinc-600'}`} />
            <span className={tunnelActive ? 'text-emerald-500' : 'text-zinc-500'}>
              {tunnelActive ? 'Online' : 'Offline'}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
