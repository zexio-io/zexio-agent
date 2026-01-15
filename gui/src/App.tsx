import { useState } from "react";
import { TunnelControl } from "./components/TunnelControl";
import { StatusCard } from "./components/StatusCard";
import { Card, CardContent, CardHeader, CardTitle } from "./components/ui/card";
import { Button } from "./components/ui/button";
import { Cpu, HardDrive, Clock, Copy, Check } from "lucide-react";
import "./App.css";

function App() {
  const [tunnelActive, setTunnelActive] = useState(false);
  const [tunnelUrl, setTunnelUrl] = useState("");
  const [copied, setCopied] = useState(false);

  const handleStartTunnel = async (provider: string, token: string) => {
    // TODO: Call Tauri backend API
    console.log("Starting tunnel:", { provider, token });
    setTunnelActive(true);
    setTunnelUrl(`https://example-${provider}.zexio.dev`);
  };

  const handleStopTunnel = async () => {
    // TODO: Call Tauri backend API
    console.log("Stopping tunnel");
    setTunnelActive(false);
    setTunnelUrl("");
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(tunnelUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white font-sans">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-3 bg-gray-800 border-b border-gray-700">
        <div className="flex items-center gap-3">
          <img src="/logo.png" alt="Zexio Logo" className="w-8 h-8 object-contain" />
          <div>
            <h1 className="text-lg font-bold tracking-tight text-white">Zexio Agent</h1>
            <p className="text-xs text-gray-400">Local Dashboard</p>
          </div>
        </div>
        <div className="px-3 py-1 text-xs font-medium text-green-400 bg-green-400/10 rounded-full border border-green-400/20">
          ● Online
        </div>
      </header>

      {/* Main Content */}
      <main className="p-6 max-w-5xl mx-auto space-y-6">
        {/* Status Cards */}
        <div className="grid grid-cols-3 gap-4">
          <StatusCard
            title="CPU Usage"
            value="12%"
            status="success"
            icon={Cpu}
          />
          <StatusCard
            title="Memory"
            value="2.4 GB"
            status="neutral"
            icon={HardDrive}
          />
          <StatusCard
            title="Uptime"
            value="3h 24m"
            status="success"
            icon={Clock}
          />
        </div>

        {/* Tunnel Control */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <TunnelControl
            isActive={tunnelActive}
            onStart={handleStartTunnel}
            onStop={handleStopTunnel}
          />

          {tunnelActive && tunnelUrl && (
            <Card variant="success">
              <CardHeader>
                <CardTitle className="text-white">Public URL</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="flex items-center gap-2 p-3 bg-gray-900 rounded-lg border border-gray-700">
                  <span className="text-sm text-blue-400 font-mono flex-1 truncate">{tunnelUrl}</span>
                  <Button
                    onClick={handleCopy}
                    size="sm"
                    variant={copied ? "outline" : "default"}
                  >
                    {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
                  </Button>
                </div>
              </CardContent>
            </Card>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
