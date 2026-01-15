import { Activity, Cloud, Clock } from "lucide-react";

interface FooterProps {
    isOnline: boolean;
    mode: "cloud" | "standalone" | null;
    cloudStatus?: "connected" | "disconnected" | "syncing";
    lastSync?: string;
}

export function Footer({ isOnline, mode, cloudStatus = "disconnected", lastSync }: FooterProps) {
    return (
        <div className="px-6 py-3 border-t border-border">
            <div className="flex items-center justify-between text-sm">
                <div className="flex items-center gap-4">
                    <div className="flex items-center gap-2">
                        <Activity className={`w-4 h-4 ${isOnline ? 'text-primary' : 'text-muted-foreground'}`} />
                        <span className="text-muted-foreground">Agent:</span>
                        <span className={isOnline ? 'text-foreground font-medium' : 'text-muted-foreground'}>
                            {isOnline ? 'Online' : 'Offline'}
                        </span>
                    </div>

                    {/* Cloud Status Indicator */}
                    {mode === "cloud" && isOnline && (
                        <div className="flex items-center gap-2 border-l border-border pl-4">
                            <Cloud className={`w-4 h-4 ${cloudStatus === 'connected' ? 'text-primary' : 'text-muted-foreground'}`} />
                            <span className="text-muted-foreground">Cloud:</span>
                            <span className={cloudStatus === 'connected' ? 'text-foreground font-medium' : 'text-muted-foreground'}>
                                {cloudStatus === 'connected' ? 'Connected' : 'Disconnected'}
                            </span>
                        </div>
                    )}
                </div>

                {/* Last Sync (Right Side) */}
                {mode === "cloud" && isOnline && lastSync && (
                    <div className="flex items-center gap-2 text-xs text-muted-foreground">
                        <Clock className="w-3 h-3" />
                        <span>Synced {lastSync}</span>
                    </div>
                )}
            </div>
        </div>
    );
}
