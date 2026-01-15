interface TunnelStatsProps {
    provider: string;
    publicUrl: string;
}

export function TunnelStats({ provider, publicUrl }: TunnelStatsProps) {
    return (
        <div className="w-full max-w-md space-y-3 mt-8">
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Provider</span>
                    <span className="text-sm font-medium capitalize">{provider}</span>
                </div>
            </div>
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between">
                    <span className="text-sm text-muted-foreground">Public URL</span>
                    <code className="text-sm font-mono text-primary truncate ml-2">
                        {publicUrl}
                    </code>
                </div>
            </div>
        </div>
    );
}
