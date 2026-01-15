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
    console.log("Starting tunnel:", { provider, token });
    setTunnelActive(true);
    setTunnelUrl(`https://example-${provider}.zexio.dev`);
  };

  const handleStopTunnel = async () => {
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
    <div className="min-h-screen bg-background text-foreground">
      {/* Header */}
      <header className="flex items-center justify-between px-4 py-3 bg-card border-b border-border">
        <div className="flex items-center gap-3">
          <img src="/logo.png" alt="Zexio Logo" className="w-8 h-8 object-contain flex-shrink-0" />
          <div>
            <h1 className="text-lg font-bold tracking-tight">Zexio Agent</h1>
            <p className="text-xs text-muted-foreground">Local Dashboard</p>
          </div>
        </div>
        <div className="px-3 py-1 text-xs font-medium text-green-400 bg-green-400/10 rounded-full border border-green-400/20 flex-shrink-0">
          ● Online
        </div>
      </header>

      {/* Main Content */}
      <main className="p-4 space-y-4">
        {/* Status Cards */}
        <div className="grid grid-cols-3 gap-3">
          <StatusCard
            title="CPU"
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
        <TunnelControl
          isActive={tunnelActive}
          onStart={handleStartTunnel}
          onStop={handleStopTunnel}
        />

        {tunnelActive && tunnelUrl && (
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-sm">Public URL</CardTitle>
            </CardHeader>
            <CardContent className="pb-4">
              <div className="flex items-center gap-2 p-2 bg-muted rounded-lg border border-border">
                <span className="text-xs text-primary font-mono flex-1 truncate">{tunnelUrl}</span>
                <Button
                  onClick={handleCopy}
                  size="sm"
                  variant={copied ? "outline" : "default"}
                >
                  {copied ? <Check className="h-3 w-3" /> : <Copy className="h-3 w-3" />}
                </Button>
              </div>
            </CardContent>
          </Card>
        )}
      </main>
    </div>
  );
}

export default App;
