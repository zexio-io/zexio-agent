
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
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
        <div className="w-full max-w-md space-y-2">
            <h2 className="text-lg font-bold mb-2">Settings</h2>

            <Tabs defaultValue={mode || "standalone"} onValueChange={(v) => onModeChange(v as "cloud" | "standalone")} className="w-full">
                <TabsList className="grid w-full grid-cols-2 mb-2 bg-muted/50 p-1 rounded-lg">
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

                <TabsContent value="standalone" className="space-y-2">
                    <div className="bg-muted/30 border border-border rounded-lg p-2 text-center">
                        <p className="text-sm text-muted-foreground">
                            Agent runs locally without cloud connection.
                            <br />
                            Suitable for local development and offline usage.
                        </p>
                    </div>
                </TabsContent>

                <TabsContent value="cloud" className="space-y-4">
                    <div className="grid gap-3">
                        <div className="grid grid-cols-3 items-center gap-4">
                            <Label className="text-right text-xs text-muted-foreground">
                                Zexio Token
                            </Label>
                            <Input
                                type="password"
                                value={token}
                                onChange={(e) => onTokenChange(e.target.value)}
                                placeholder="Enter your Zexio token..."
                                className="col-span-2 h-8 text-xs bg-card border-border"
                            />
                        </div>

                        <div className="grid grid-cols-3 items-center gap-4">
                            <Label className="text-right text-xs text-muted-foreground">
                                Node ID
                            </Label>
                            <Input
                                type="text"
                                value={nodeId}
                                onChange={(e) => onNodeIdChange(e.target.value)}
                                placeholder="Enter Node ID (e.g. node-123)"
                                className="col-span-2 h-8 text-xs bg-card border-border"
                            />
                        </div>
                    </div>

                    <Button
                        onClick={onSave}
                        className="w-full h-8 text-xs bg-amber-400 hover:bg-amber-500 text-black font-semibold mt-4 transition-colors"
                    >
                        Save Settings
                    </Button>


                </TabsContent>
            </Tabs>
        </div>
    );
}
