import { useState } from "react";

interface TunnelControlProps {
    onStart: (provider: string, token: string) => void;
    onStop: () => void;
    isActive: boolean;
}

export function TunnelControl({ onStart, onStop, isActive }: TunnelControlProps) {
    const [provider, setProvider] = useState("cloudflare");
    const [token, setToken] = useState("");
    const [showTokenInput, setShowTokenInput] = useState(false);

    const handleStart = () => {
        if (!token.trim()) {
            setShowTokenInput(true);
            return;
        }
        onStart(provider, token);
    };

    return (
        <div className="p-6 bg-gray-800 rounded-xl border border-gray-700">
            <div className="flex items-center justify-between mb-4">
                <div>
                    <h3 className="text-sm font-medium text-gray-400 mb-1">Tunnel Status</h3>
                    <div className="flex items-center gap-2">
                        <div className={`w-2 h-2 rounded-full ${isActive ? 'bg-green-500' : 'bg-gray-600'}`} />
                        <span className="text-2xl font-bold text-white">
                            {isActive ? "Active" : "Inactive"}
                        </span>
                    </div>
                </div>

                {isActive ? (
                    <button
                        onClick={onStop}
                        className="px-4 py-2 bg-red-600 hover:bg-red-500 text-white text-sm font-semibold rounded-lg transition-colors"
                    >
                        Stop Tunnel
                    </button>
                ) : (
                    <button
                        onClick={handleStart}
                        className="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold rounded-lg transition-colors"
                    >
                        Start Tunnel
                    </button>
                )}
            </div>

            {!isActive && (
                <div className="space-y-3 pt-4 border-t border-gray-700">
                    <div>
                        <label className="block text-xs font-medium text-gray-400 mb-2">
                            Provider
                        </label>
                        <select
                            value={provider}
                            onChange={(e) => setProvider(e.target.value)}
                            className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white text-sm focus:outline-none focus:border-blue-500"
                        >
                            <option value="cloudflare">Cloudflare Tunnel</option>
                            <option value="pangolin">Pangolin Tunnel</option>
                        </select>
                    </div>

                    {(showTokenInput || token) && (
                        <div>
                            <label className="block text-xs font-medium text-gray-400 mb-2">
                                Auth Token
                            </label>
                            <input
                                type="password"
                                value={token}
                                onChange={(e) => setToken(e.target.value)}
                                placeholder="Enter your tunnel token..."
                                className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white text-sm placeholder-gray-500 focus:outline-none focus:border-blue-500"
                            />
                        </div>
                    )}
                </div>
            )}
        </div>
    );
}
