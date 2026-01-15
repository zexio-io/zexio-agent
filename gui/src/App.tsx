import { useState, useEffect } from "react";
import { OnboardingScreen } from "./components/OnboardingScreen";
import { Header } from "./components/Header";
import { Footer } from "./components/Footer";
import { LogoBrand } from "./components/LogoBrand";
import { TunnelToggle } from "./components/TunnelToggle";
import { TunnelStats } from "./components/TunnelStats";
import { SettingsPanel } from "./components/SettingsPanel";

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

  // Load config from localStorage on mount
  useEffect(() => {
    const savedConfig = localStorage.getItem("zexio-config");
    if (savedConfig) {
      setConfig(JSON.parse(savedConfig));
    }
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
    <div className="h-screen bg-zinc-950 text-white flex flex-col">
      <Header onSettingsClick={() => setShowSettings(!showSettings)} />

      <div className="flex-1 flex flex-col items-center justify-center px-6">
        {!showSettings ? (
          <>
            <LogoBrand size="lg" />

            <div className="mb-12" />

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

            {/* Show deployment mode badge */}
            <div className="mt-8">
              <div className="px-3 py-1 text-xs font-medium bg-zinc-800 text-zinc-400 rounded-full">
                {config.mode === "cloud" ? "☁️ Cloud Mode" : "🏠 Standalone Mode"}
              </div>
            </div>
          </>
        ) : (
          <SettingsPanel
            provider={provider}
            token={token}
            onProviderChange={setProvider}
            onTokenChange={setToken}
            onSave={handleSaveSettings}
          />
        )}
      </div>

      <Footer isOnline={tunnelActive} />
    </div>
  );
}

export default App;
