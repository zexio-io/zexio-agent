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
    active_services: number;
    active_addons: number;
}

interface CloudStatsData {
    apps: {
        active: number;
        stopped: number;
        crashed: number;
    };
    services: {
        active: number;
        stopped: number;
        crashed: number;
    };
    addons: {
        enabled: number;
        installed: number;
    };
}

interface SystemStatsProps {
    stats: SystemStatsData;
    cloudStats: CloudStatsData;
}


export function SystemStats({ stats, cloudStats }: SystemStatsProps) {
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
        <div className="space-y-6">
            {/* Section 1: System Resources (3 cards) */}
            <div>
                <h3 className="text-sm font-medium text-muted-foreground mb-3">System Resources</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
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
            </div>

            {/* Section 2: Management (3 cards) */}
            <div>
                <h3 className="text-sm font-medium text-muted-foreground mb-3">Management</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {/* Apps Card */}
                    <div className="bg-card border border-border rounded-lg p-4 cursor-pointer hover:bg-muted/10 transition-colors group">
                        <div className="flex items-center justify-between mb-2">
                            <span className="text-xs text-muted-foreground group-hover:text-foreground">Apps</span>
                            <div className="w-4 h-4 rounded-full bg-emerald-500/20 text-emerald-500 flex items-center justify-center text-[10px] font-bold">
                                A
                            </div>
                        </div>
                        <div className="flex items-baseline gap-1 mt-1">
                            <span className="text-2xl font-bold text-foreground">{cloudStats.apps.active + cloudStats.apps.stopped + cloudStats.apps.crashed}</span>
                            <span className="text-xs text-muted-foreground">Total</span>
                        </div>
                        <div className="flex gap-2 mt-3 text-[10px]">
                            <div className="flex items-center gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-500" />
                                <span className="text-muted-foreground">Active: {cloudStats.apps.active}</span>
                            </div>
                            <div className="flex items-center gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-muted-foreground" />
                                <span className="text-muted-foreground">Stop: {cloudStats.apps.stopped}</span>
                            </div>
                            <div className="flex items-center gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-red-500" />
                                <span className="text-red-500 font-medium">Crash: {cloudStats.apps.crashed}</span>
                            </div>
                        </div>
                    </div>

                    {/* Services Card */}
                    <div className="bg-card border border-border rounded-lg p-4 cursor-pointer hover:bg-muted/10 transition-colors group">
                        <div className="flex items-center justify-between mb-2">
                            <span className="text-xs text-muted-foreground group-hover:text-foreground">Services</span>
                            <div className="w-4 h-4 rounded-full bg-orange-500/20 text-orange-500 flex items-center justify-center text-[10px] font-bold">
                                S
                            </div>
                        </div>
                        <div className="flex items-baseline gap-1 mt-1">
                            <span className="text-2xl font-bold text-foreground">{cloudStats.services.active + cloudStats.services.stopped + cloudStats.services.crashed}</span>
                            <span className="text-xs text-muted-foreground group-hover:text-primary transition-colors">Database & Tools</span>
                        </div>
                        <div className="flex gap-2 mt-3 text-[10px]">
                            <div className="flex items-center gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-emerald-500" />
                                <span className="text-muted-foreground">Active: {cloudStats.services.active}</span>
                            </div>
                            <div className="flex items-center gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-muted-foreground" />
                                <span className="text-muted-foreground">Stop: {cloudStats.services.stopped}</span>
                            </div>
                            <div className="flex items-center gap-1">
                                <div className="w-1.5 h-1.5 rounded-full bg-red-500" />
                                <span className="text-red-500 font-medium">Crash: {cloudStats.services.crashed}</span>
                            </div>
                        </div>
                    </div>

                    {/* Addons Card */}
                    <div className="bg-card border border-border rounded-lg p-4 cursor-pointer hover:bg-muted/10 transition-colors group">
                        <div className="flex items-center justify-between mb-2">
                            <span className="text-xs text-muted-foreground group-hover:text-foreground">Addons</span>
                            <div className="w-4 h-4 rounded-full bg-purple-500/20 text-purple-500 flex items-center justify-center text-[10px] font-bold">
                                P
                            </div>
                        </div>

                        <div className="flex items-baseline gap-1 mt-1">
                            <span className="text-2xl font-bold text-foreground">{cloudStats.addons.enabled}</span>
                            <span className="text-xl text-muted-foreground">/</span>
                            <span className="text-xl text-muted-foreground">{cloudStats.addons.installed}</span>
                            <span className="text-xs text-muted-foreground ml-2 group-hover:text-primary transition-colors">Addons Plugins</span>
                        </div>

                        <div className="mt-2 h-1 bg-muted rounded-full overflow-hidden flex">
                            <div
                                className="h-full bg-purple-500 transition-all duration-300"
                                style={{ width: `${cloudStats.addons.installed > 0 ? (cloudStats.addons.enabled / cloudStats.addons.installed) * 100 : 0}%` }}
                            />
                        </div>
                        <p className="text-[10px] text-muted-foreground mt-2">
                            {cloudStats.addons.enabled} Enabled from {cloudStats.addons.installed} Installed
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
}
