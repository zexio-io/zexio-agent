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
    <div className="flex flex-col h-screen bg-background text-foreground overflow-hidden">
      {/* Header - Fixed */}
      <header className="flex-shrink-0 flex items-center justify-between px-4 py-3 bg-card border-b border-border">
        <div className="flex items-center gap-3 min-w-0">
          <img src="/logo.png" alt="Zexio" className="w-8 h-8 flex-shrink-0" />
          <div className="min-w-0">
            <h1 className="text-base font-bold truncate">Zexio Agent</h1>
            <p className="text-xs text-muted-foreground">Local Dashboard</p>
          </div>
        </div>
        <div className="flex-shrink-0 px-2.5 py-1 text-xs font-medium text-green-400 bg-green-400/10 rounded-full border border-green-400/20">
          ● Online
        </div>
      </header>

      {/* Main Content - Scrollable */}
      <main className="flex-1 overflow-y-auto">
        <div className="p-4 space-y-4 max-w-4xl mx-auto">
          {/* Status Cards Row */}
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

          {/* Public URL Card - Only shown when tunnel is active */}
          {tunnelActive && tunnelUrl && (
            <Card>
              <CardHeader className="pb-3">
                <CardTitle className="text-sm">Public URL</CardTitle>
              </CardHeader>
              <CardContent className="pb-4">
                <div className="flex items-center gap-2 p-2.5 bg-muted rounded-lg border border-border">
                  <code className="text-xs text-primary font-mono flex-1 truncate">
                    {tunnelUrl}
                  </code>
                  <Button
                    onClick={handleCopy}
                    size="sm"
                    variant={copied ? "outline" : "default"}
                    className="flex-shrink-0"
                  >
                    {copied ? (
                      <Check className="h-3.5 w-3.5" />
                    ) : (
                      <Copy className="h-3.5 w-3.5" />
                    )}
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
