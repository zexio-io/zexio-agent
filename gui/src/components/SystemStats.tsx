import { Cpu, HardDrive, Database } from "lucide-react";

interface SystemStatsData {
    cpu_usage: number;
    memory_used: number;
    memory_total: number;
    memory_percent: number;
    disk_used: number;
    disk_total: number;
    disk_percent: number;
    total_projects: number;
}

interface SystemStatsProps {
    stats: SystemStatsData;
}

export function SystemStats({ stats }: SystemStatsProps) {
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
                    <Cpu className={`w-4 h-4 ${getStatusColor(stats.cpu_usage)}`} />
                </div>
                <p className={`text-2xl font-bold ${getStatusColor(stats.cpu_usage)}`}>
                    {stats.cpu_usage.toFixed(1)}%
                </p>
                <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        className={`h-full ${getStatusColor(stats.cpu_usage)} bg-current transition-all duration-300`}
                        style={{ width: `${stats.cpu_usage}%` }}
                    />
                </div>
            </div>

            {/* Memory */}
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between mb-2">
                    <span className="text-xs text-muted-foreground">Memory</span>
                    <HardDrive className={`w-4 h-4 ${getStatusColor(stats.memory_percent)}`} />
                </div>
                <p className={`text-2xl font-bold ${getStatusColor(stats.memory_percent)}`}>
                    {stats.memory_percent.toFixed(1)}%
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                    {formatBytes(stats.memory_used)} / {formatBytes(stats.memory_total)}
                </p>
                <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        className={`h-full ${getStatusColor(stats.memory_percent)} bg-current transition-all duration-300`}
                        style={{ width: `${stats.memory_percent}%` }}
                    />
                </div>
            </div>

            {/* Storage */}
            <div className="bg-card border border-border rounded-lg p-4">
                <div className="flex items-center justify-between mb-2">
                    <span className="text-xs text-muted-foreground">Storage</span>
                    <Database className={`w-4 h-4 ${getStatusColor(stats.disk_percent)}`} />
                </div>
                <p className={`text-2xl font-bold ${getStatusColor(stats.disk_percent)}`}>
                    {stats.disk_percent.toFixed(1)}%
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                    {formatBytes(stats.disk_used)} / {formatBytes(stats.disk_total)}
                </p>
                <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        className={`h-full ${getStatusColor(stats.disk_percent)} bg-current transition-all duration-300`}
                        style={{ width: `${stats.disk_percent}%` }}
                    />
                </div>
            </div>
        </div>
    );
}
