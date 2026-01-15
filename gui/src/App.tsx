import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { OnboardingScreen } from "./components/OnboardingScreen";
import { Header } from "./components/Header";
import { Footer } from "./components/Footer";
import { LogoBrand } from "./components/LogoBrand";
import { TunnelToggle } from "./components/TunnelToggle";
import { TunnelStats } from "./components/TunnelStats";
import { SettingsPanel } from "./components/SettingsPanel";
import { SystemStats } from "./components/SystemStats";

type DeploymentMode = "cloud" | "standalone" | null;

interface AppConfig {
  mode: DeploymentMode;
  token?: string;
  nodeId?: string;
  apiPort?: number;
  meshPort?: number;
}

interface SystemStatsData {
  cpu: number;
  memory: {
    used: number;
    total: number;
  };
  storage: {
    used: number;
    total: number;
  };
}

function App() {
  const [config, setConfig] = useState<AppConfig>({ mode: null });
  const [tunnelActive, setTunnelActive] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [provider, setProvider] = useState("cloudflare");
  const [token, setToken] = useState("");
  const [agentOnline, setAgentOnline] = useState(false);
  const [systemStats, setSystemStats] = useState<SystemStatsData>({
    cpu: 0,
    memory: { used: 0, total: 8 * 1024 * 1024 * 1024 },
    storage: { used: 0, total: 256 * 1024 * 1024 * 1024 },
  });

  // Load config from localStorage on mount
  useEffect(() => {
    const savedConfig = localStorage.getItem("zexio-config");
    if (savedConfig) {
      setConfig(JSON.parse(savedConfig));
    }

    // Check agent health
    checkAgentHealth();

    // Poll system stats every 2 seconds
    const statsInterval = setInterval(fetchSystemStats, 2000);

    return () => clearInterval(statsInterval);
  }, []);

  const checkAgentHealth = async () => {
    try {
      await invoke("health_check");
      setAgentOnline(true);
    } catch (error) {
      console.error("Agent not running:", error);
      setAgentOnline(false);
    }
  };

  const fetchSystemStats = async () => {
    try {
      const stats = await invoke<SystemStatsData>("get_system_stats");
      setSystemStats(stats);
    } catch (error) {
      console.error("Failed to fetch stats:", error);
    }
  };

  const handleOnboardingComplete = (newConfig: Omit<AppConfig, "mode"> & { mode: "cloud" | "standalone" }) => {
    const fullConfig = { ...newConfig };
    setConfig(fullConfig);
    localStorage.setItem("zexio-config", JSON.stringify(fullConfig));
  };

  const handleToggle = async () => {
    if (!tunnelActive) {
      if (!token.trim()) {
        setShowSettings(true);
        return;
      }

      // Start tunnel
      try {
        await invoke("start_tunnel", { provider, token });
        setTunnelActive(true);
      } catch (error) {
        console.error("Failed to start tunnel:", error);
        alert(`Failed to start tunnel: ${error}`);
      }
    } else {
      // Stop tunnel
      try {
        await invoke("stop_tunnel");
        setTunnelActive(false);
      } catch (error) {
        console.error("Failed to stop tunnel:", error);
        alert(`Failed to stop tunnel: ${error}`);
      }
    }
  };

  const handleSaveSettings = () => {
    setShowSettings(false);
  };

  const publicUrl = `https://example-${provider}.zexio.dev`;

  // Show onboarding if not configured
  if (!config.mode) {
    return <OnboardingScreen onComplete={handleOnboardingComplete} />;
  }

  // Show main app
  return (
    <div className="h-screen bg-background text-foreground flex flex-col">
      <Header onSettingsClick={() => setShowSettings(!showSettings)} />

      <div className="flex-1 overflow-y-auto">
        <div className="p-4 space-y-4 max-w-4xl mx-auto">
          {!showSettings ? (
            <>
              {/* Agent Status Banner */}
              {!agentOnline && (
                <div className="bg-destructive/10 border border-destructive/20 rounded-lg p-3 text-center">
                  <p className="text-sm text-destructive">
                    ⚠️ Agent not running. Please start the agent with <code className="bg-muted px-1 rounded">cargo run</code>
                  </p>
                </div>
              )}

              {/* System Stats */}
              {agentOnline && (
                <SystemStats
                  cpu={systemStats.cpu}
                  memory={systemStats.memory}
                  storage={systemStats.storage}
                />
              )}

              {/* Logo & Tunnel */}
              <div className="flex flex-col items-center py-8">
                <LogoBrand size="md" />

                <div className="mt-8 mb-4" />

                <TunnelToggle
                  isActive={tunnelActive}
                  onToggle={handleToggle}
                />

                {tunnelActive && (
                  <TunnelStats
                    provider={provider}
                    publicUrl={publicUrl}
                  />
                )}

                {/* Deployment mode badge */}
                <div className="mt-8">
                  <div className="px-3 py-1 text-xs font-medium bg-muted text-muted-foreground rounded-full">
                    {config.mode === "cloud" ? "☁️ Cloud Mode" : "🏠 Standalone Mode"}
                  </div>
                </div>
              </div>
            </>
          ) : (
            <div className="flex items-center justify-center min-h-[calc(100vh-8rem)]">
              <SettingsPanel
                provider={provider}
                token={token}
                onProviderChange={setProvider}
                onTokenChange={setToken}
                onSave={handleSaveSettings}
              />
            </div>
          )}
        </div>
      </div>

      <Footer isOnline={agentOnline && tunnelActive} />
    </div>
  );
}

export default App;
