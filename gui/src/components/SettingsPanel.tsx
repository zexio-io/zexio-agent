
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";

interface SettingsPanelProps {
    token: string;
    nodeId: string;
    onTokenChange: (value: string) => void;
    onNodeIdChange: (value: string) => void;
    mode: "cloud" | "standalone" | null;
    onModeChange: (mode: "cloud" | "standalone") => void;
    onSave: () => void;
}

export function SettingsPanel({
    token,
    nodeId,
    onTokenChange,
    onNodeIdChange,
    mode,
    onModeChange,
    onSave
}: SettingsPanelProps) {
    return (
        <div className="w-full max-w-md space-y-4">
            <h2 className="text-2xl font-bold mb-6">Settings</h2>

            <Tabs defaultValue={mode || "standalone"} onValueChange={(v) => onModeChange(v as "cloud" | "standalone")} className="w-full">
                <TabsList className="grid w-full grid-cols-2 mb-4 bg-muted/50 p-1 rounded-lg">
                    <TabsTrigger
                        value="standalone"
                        className="data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm rounded-md py-2 text-sm font-medium transition-all"
                    >
                        🏠 Standalone
                    </TabsTrigger>
                    <TabsTrigger
                        value="cloud"
                        className="data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm rounded-md py-2 text-sm font-medium transition-all"
                    >
                        ☁️ Cloud
                    </TabsTrigger>
                </TabsList>

                <TabsContent value="standalone" className="space-y-4">
                    <div className="bg-muted/30 border border-border rounded-lg p-4 text-center">
                        <p className="text-sm text-muted-foreground">
                            Agent runs locally without cloud connection.
                            <br />
                            Suitable for local development and offline usage.
                        </p>
                    </div>
                </TabsContent>

                <TabsContent value="cloud" className="space-y-4">
                    <div>
                        <label className="block text-sm text-muted-foreground mb-2">
                            Zexio Token
                        </label>
                        <input
                            type="password"
                            value={token}
                            onChange={(e) => onTokenChange(e.target.value)}
                            placeholder="Enter your Zexio token..."
                            className="w-full px-4 py-3 bg-card border border-border rounded-lg text-white placeholder-muted-foreground focus:outline-none focus:border-primary"
                        />
                    </div>

                    <div>
                        <label className="block text-sm text-muted-foreground mb-2">
                            Node ID
                        </label>
                        <input
                            type="text"
                            value={nodeId}
                            onChange={(e) => onNodeIdChange(e.target.value)}
                            placeholder="Enter Node ID (e.g. node-123)"
                            className="w-full px-4 py-3 bg-card border border-border rounded-lg text-white placeholder-muted-foreground focus:outline-none focus:border-primary"
                        />
                    </div>


                </TabsContent>
            </Tabs>

            <button
                onClick={onSave}
                className="w-full py-3 bg-primary hover:bg-primary/90 text-primary-foreground rounded-lg font-medium transition-colors mt-6"
            >
                Save Settings
            </button>
        </div>
    );
}
