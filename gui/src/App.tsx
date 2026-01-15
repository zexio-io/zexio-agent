import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { OnboardingScreen } from "./components/OnboardingScreen";
import { Header } from "./components/Header";
import { Footer } from "./components/Footer";
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

// Matches SystemStats in lib.rs / ROUTES.md
interface SystemStatsData {
  cpu_usage: number;
  memory_used: number;
  memory_total: number;
  memory_percent: number;
  disk_used: number;
  disk_total: number;
  disk_percent: number;
  total_projects: number;
}

function App() {
  const [config, setConfig] = useState<AppConfig>({ mode: null });

  const [showSettings, setShowSettings] = useState(false);
  const [token, setToken] = useState("");
  const [agentOnline, setAgentOnline] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [systemStats, setSystemStats] = useState<SystemStatsData>({
    cpu_usage: 0,
    memory_used: 0,
    memory_total: 8 * 1024 * 1024 * 1024,
    memory_percent: 0,
    disk_used: 0,
    disk_total: 256 * 1024 * 1024 * 1024,
    disk_percent: 0,
    total_projects: 0,
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
      setErrorMsg(null);
    } catch (error) {
      console.error("Agent not running:", error);
      setAgentOnline(false);
      setErrorMsg(String(error));
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



  const handleSaveSettings = () => {
    setShowSettings(false);
  };



  // Show onboarding if not configured
  if (!config.mode) {
    return <OnboardingScreen onComplete={handleOnboardingComplete} />;
  }

  // Show main app
  return (
    <div className="h-screen bg-background text-foreground flex flex-col">
      <Header
        onSettingsClick={() => setShowSettings(!showSettings)}
        onLogoClick={() => setShowSettings(false)}
        mode={config.mode}
      />

      <div className="flex-1 overflow-y-auto">
        <div className="p-4 space-y-4 max-w-4xl mx-auto">
          {!showSettings ? (
            <>
              {/* Agent Status Banner */}
              {!agentOnline && (
                <div className="bg-destructive/10 border border-destructive/20 rounded-lg p-3 text-center">
                  <p className="text-sm text-destructive font-bold">
                    ⚠️ Agent not running
                  </p>
                  {errorMsg && (
                    <p className="text-xs text-destructive/80 mt-1 font-mono">
                      Error: {errorMsg}
                    </p>
                  )}
                  <p className="text-xs text-muted-foreground mt-2">
                    Please start the agent with <code className="bg-muted px-1 rounded">cargo run</code>
                  </p>
                </div>
              )}

              {/* System Stats */}
              {agentOnline && (
                <SystemStats stats={systemStats} />
              )}

              {/* Logo (Centered, larger version removed or kept? User said "[Zexio Logo] [Mode]" in toolbar) */}
              {/* If we moved logo to header, maybe we don't need it big in center anymore. 
                  But user only said "standalone mode pindah ke toolbar", 
                  implies removing the badge from body. 
                  Usually dashboard has a hero section or just stats.
                  I will remove the Big Logo + Badge section entirely as it duplicates header somewhat and user wants removal of badge from there. 
              */}

            </>
          ) : (
            <div className="flex justify-center pt-10">
              <SettingsPanel
                token={token}
                nodeId={config.nodeId || ""}
                onTokenChange={setToken}
                onNodeIdChange={(nodeId) => {
                  const newConfig = { ...config, nodeId };
                  setConfig(newConfig);
                  localStorage.setItem("zexio-config", JSON.stringify(newConfig));
                }}
                mode={config.mode}
                onModeChange={(mode) => {
                  const newConfig = { ...config, mode };
                  setConfig(newConfig);
                  localStorage.setItem("zexio-config", JSON.stringify(newConfig));
                }}
                onSave={handleSaveSettings}
              />
            </div>
          )}
        </div>
      </div>

      <Footer
        isOnline={agentOnline}
        mode={config.mode}
        cloudStatus="connected" // TODO: Real status from backend
        lastSync="just now"     // TODO: Real timestamp
      />
    </div >
  );
}

export default App;
