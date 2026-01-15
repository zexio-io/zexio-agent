import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "./ui/select";

interface SettingsPanelProps {
    provider: string;
    token: string;
    onProviderChange: (value: string) => void;
    onTokenChange: (value: string) => void;
    onSave: () => void;
}

export function SettingsPanel({
    provider,
    token,
    onProviderChange,
    onTokenChange,
    onSave
}: SettingsPanelProps) {
    return (
        <div className="w-full max-w-md space-y-4">
            <h2 className="text-2xl font-bold mb-6">Settings</h2>

            <div>
                <label className="block text-sm text-muted-foreground mb-2">
                    Tunnel Provider
                </label>
                <Select value={provider} onValueChange={onProviderChange}>
                    <SelectTrigger className="w-full bg-card border-border text-white">
                        <SelectValue placeholder="Select provider" />
                    </SelectTrigger>
                    <SelectContent className="bg-card border-border">
                        <SelectItem value="cloudflare" className="text-white focus:bg-border focus:text-white">
                            Cloudflare Tunnel
                        </SelectItem>
                        <SelectItem value="pangolin" className="text-white focus:bg-border focus:text-white">
                            Pangolin Tunnel
                        </SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div>
                <label className="block text-sm text-muted-foreground mb-2">
                    Authentication Token
                </label>
                <input
                    type="password"
                    value={token}
                    onChange={(e) => onTokenChange(e.target.value)}
                    placeholder="Enter your tunnel token..."
                    className="w-full px-4 py-3 bg-card border border-border rounded-lg text-white placeholder-muted-foreground focus:outline-none focus:border-primary"
                />
            </div>

            <button
                onClick={onSave}
                className="w-full py-3 bg-primary hover:bg-primary/90 text-primary-foreground rounded-lg font-medium transition-colors"
            >
                Save Settings
            </button>
        </div>
    );
}
