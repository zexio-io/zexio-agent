import { useState, useEffect } from "react";
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

function App() {
  const [config, setConfig] = useState<AppConfig>({ mode: null });
  const [tunnelActive, setTunnelActive] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [provider, setProvider] = useState("cloudflare");
  const [token, setToken] = useState("");

  // Mock system stats (will be replaced with real data from Tauri)
  const [systemStats, setSystemStats] = useState({
    cpu: 12.5,
    memory: {
      used: 2.4 * 1024 * 1024 * 1024, // 2.4 GB in bytes
      total: 8 * 1024 * 1024 * 1024,  // 8 GB in bytes
    },
    storage: {
      used: 45 * 1024 * 1024 * 1024,  // 45 GB in bytes
      total: 256 * 1024 * 1024 * 1024, // 256 GB in bytes
    },
  });

  // Load config from localStorage on mount
  useEffect(() => {
    const savedConfig = localStorage.getItem("zexio-config");
    if (savedConfig) {
      setConfig(JSON.parse(savedConfig));
    }

    // Simulate real-time stats updates (will be replaced with actual API calls)
    const interval = setInterval(() => {
      setSystemStats(prev => ({
        cpu: Math.max(5, Math.min(95, prev.cpu + (Math.random() - 0.5) * 10)),
        memory: prev.memory,
        storage: prev.storage,
      }));
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  const handleOnboardingComplete = (newConfig: Omit<AppConfig, "mode"> & { mode: "cloud" | "standalone" }) => {
    const fullConfig = { ...newConfig };
    setConfig(fullConfig);
    localStorage.setItem("zexio-config", JSON.stringify(fullConfig));
  };

  const handleToggle = () => {
    if (!tunnelActive && !token.trim()) {
      setShowSettings(true);
      return;
    }
    setTunnelActive(!tunnelActive);
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
              {/* System Stats */}
              <SystemStats
                cpu={systemStats.cpu}
                memory={systemStats.memory}
                storage={systemStats.storage}
              />

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

      <Footer isOnline={tunnelActive} />
    </div>
  );
}

export default App;
