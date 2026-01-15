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
                <label className="block text-sm text-zinc-400 mb-2">
                    Tunnel Provider
                </label>
                <Select value={provider} onValueChange={onProviderChange}>
                    <SelectTrigger className="w-full bg-zinc-900 border-zinc-800 text-white">
                        <SelectValue placeholder="Select provider" />
                    </SelectTrigger>
                    <SelectContent className="bg-zinc-900 border-zinc-800">
                        <SelectItem value="cloudflare" className="text-white focus:bg-zinc-800 focus:text-white">
                            Cloudflare Tunnel
                        </SelectItem>
                        <SelectItem value="pangolin" className="text-white focus:bg-zinc-800 focus:text-white">
                            Pangolin Tunnel
                        </SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div>
                <label className="block text-sm text-zinc-400 mb-2">
                    Authentication Token
                </label>
                <input
                    type="password"
                    value={token}
                    onChange={(e) => onTokenChange(e.target.value)}
                    placeholder="Enter your tunnel token..."
                    className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500"
                />
            </div>

            <button
                onClick={onSave}
                className="w-full py-3 bg-blue-600 hover:bg-blue-500 rounded-lg font-medium transition-colors"
            >
                Save Settings
            </button>
        </div>
    );
}
