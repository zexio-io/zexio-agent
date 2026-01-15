import { useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { Button } from "./ui/button";
import { Activity, Loader2 } from "lucide-react";

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
        <Card>
            <CardHeader className="pb-3 pt-4 px-4">
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                        <Activity className={`h-4 w-4 ${isActive ? 'text-green-400' : 'text-gray-400'}`} />
                        <CardTitle className="text-sm text-white">Tunnel</CardTitle>
                    </div>
                    <div className={`flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium ${isActive
                            ? 'bg-green-400/10 text-green-400 border border-green-400/20'
                            : 'bg-gray-700 text-gray-400'
                        }`}>
                        <div className={`w-1.5 h-1.5 rounded-full ${isActive ? 'bg-green-400' : 'bg-gray-500'}`} />
                        {isActive ? "Active" : "Inactive"}
                    </div>
                </div>
                <CardDescription className="text-gray-400 text-xs mt-1">
                    {isActive ? "Tunnel is running" : "Start a secure tunnel"}
                </CardDescription>
            </CardHeader>

            <CardContent className="px-4 pb-4">
                {!isActive && (
                    <div className="space-y-2 mb-3">
                        <div>
                            <label className="block text-xs font-medium text-gray-400 mb-1.5">
                                Provider
                            </label>
                            <select
                                value={provider}
                                onChange={(e) => setProvider(e.target.value)}
                                className="w-full px-2 py-1.5 bg-gray-900 border border-gray-700 rounded-lg text-white text-xs focus:outline-none focus:border-blue-500"
                            >
                                <option value="cloudflare">Cloudflare Tunnel</option>
                                <option value="pangolin">Pangolin Tunnel</option>
                            </select>
                        </div>

                        {(showTokenInput || token) && (
                            <div>
                                <label className="block text-xs font-medium text-gray-400 mb-1.5">
                                    Auth Token
                                </label>
                                <input
                                    type="password"
                                    value={token}
                                    onChange={(e) => setToken(e.target.value)}
                                    placeholder="Enter your tunnel token..."
                                    className="w-full px-2 py-1.5 bg-gray-900 border border-gray-700 rounded-lg text-white text-xs placeholder-gray-500 focus:outline-none focus:border-blue-500"
                                />
                            </div>
                        )}
                    </div>
                )}

                {isActive ? (
                    <Button onClick={onStop} variant="destructive" className="w-full h-8 text-xs">
                        Stop Tunnel
                    </Button>
                ) : (
                    <Button onClick={handleStart} className="w-full h-8 text-xs">
                        Start Tunnel
                    </Button>
                )}
            </CardContent>
        </Card>
    );
}
