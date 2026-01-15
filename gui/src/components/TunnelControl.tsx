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
        <Card variant={isActive ? "success" : "default"}>
            <CardHeader>
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                        <Activity className={`h-4 w-4 ${isActive ? 'text-green-400' : 'text-gray-400'}`} />
                        <CardTitle className="text-white">Tunnel</CardTitle>
                    </div>
                    <div className={`flex items-center gap-2 px-2 py-1 rounded-full text-xs font-medium ${isActive
                            ? 'bg-green-400/10 text-green-400 border border-green-400/20'
                            : 'bg-gray-700 text-gray-400'
                        }`}>
                        <div className={`w-1.5 h-1.5 rounded-full ${isActive ? 'bg-green-400' : 'bg-gray-500'}`} />
                        {isActive ? "Active" : "Inactive"}
                    </div>
                </div>
                <CardDescription className="text-gray-400">
                    {isActive ? "Tunnel is running" : "Start a secure tunnel to expose your services"}
                </CardDescription>
            </CardHeader>

            <CardContent>
                {!isActive && (
                    <div className="space-y-3 mb-4">
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

                {isActive ? (
                    <Button onClick={onStop} variant="destructive" className="w-full">
                        Stop Tunnel
                    </Button>
                ) : (
                    <Button onClick={handleStart} className="w-full">
                        Start Tunnel
                    </Button>
                )}
            </CardContent>
        </Card>
    );
}
