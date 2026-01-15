import { Cpu, HardDrive, Database } from "lucide-react";

interface SystemStatsProps {
    cpu: number;
    memory: {
        used: number;
        total: number;
    };
    storage: {
        used: number;
        total: number;
    };
}

export function SystemStats({ cpu, memory, storage }: SystemStatsProps) {
    const memoryPercent = (memory.used / memory.total) * 100;
    const storagePercent = (storage.used / storage.total) * 100;

    const formatBytes = (bytes: number) => {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
    };

    const getStatusColor = (percent: number) => {
        if (percent < 60) return 'text-primary';
        if (percent < 80) return 'text-yellow-500';
        return 'text-red-500';
    };

    return (
        <div className="grid grid-cols-3 gap-3">
            {/* CPU */}
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between mb-2">
                    <span className="text-xs text-muted-foreground">CPU</span>
                    <Cpu className={`w-4 h-4 ${getStatusColor(cpu)}`} />
                </div>
                <p className={`text-2xl font-bold ${getStatusColor(cpu)}`}>
                    {cpu.toFixed(1)}%
                </p>
                <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        className={`h-full ${getStatusColor(cpu)} bg-current transition-all duration-300`}
                        style={{ width: `${cpu}%` }}
                    />
                </div>
            </div>

            {/* Memory */}
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between mb-2">
                    <span className="text-xs text-muted-foreground">Memory</span>
                    <HardDrive className={`w-4 h-4 ${getStatusColor(memoryPercent)}`} />
                </div>
                <p className={`text-2xl font-bold ${getStatusColor(memoryPercent)}`}>
                    {memoryPercent.toFixed(1)}%
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                    {formatBytes(memory.used)} / {formatBytes(memory.total)}
                </p>
                <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        className={`h-full ${getStatusColor(memoryPercent)} bg-current transition-all duration-300`}
                        style={{ width: `${memoryPercent}%` }}
                    />
                </div>
            </div>

            {/* Storage */}
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between mb-2">
                    <span className="text-xs text-muted-foreground">Storage</span>
                    <Database className={`w-4 h-4 ${getStatusColor(storagePercent)}`} />
                </div>
                <p className={`text-2xl font-bold ${getStatusColor(storagePercent)}`}>
                    {storagePercent.toFixed(1)}%
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                    {formatBytes(storage.used)} / {formatBytes(storage.total)}
                </p>
                <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        className={`h-full ${getStatusColor(storagePercent)} bg-current transition-all duration-300`}
                        style={{ width: `${storagePercent}%` }}
                    />
                </div>
            </div>
        </div>
    );
}
