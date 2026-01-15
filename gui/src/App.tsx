import { useState } from "react";
import { Header } from "./components/Header";
import { Footer } from "./components/Footer";
import { LogoBrand } from "./components/LogoBrand";
import { TunnelToggle } from "./components/TunnelToggle";
import { TunnelStats } from "./components/TunnelStats";
import { SettingsPanel } from "./components/SettingsPanel";

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

  const handleSaveSettings = () => {
    setShowSettings(false);
  };

  const publicUrl = `https://example-${provider}.zexio.dev`;

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
