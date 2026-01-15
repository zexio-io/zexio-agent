import { Cpu, HardDrive, Database } from "lucide-react";

interface SystemStatsData {
    cpu_usage: number;
    memory_used: number;
    memory_total: number;
    memory_percent: number;
    disk_used: number;
    disk_total: number;
    disk_percent: number;
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

const statsFontSizes = {
    title: "text-[10px]",
    value: "text-[9px]",
    subValue: "text-[8px]"
}


const ResourceCard = ({ title, icon: Icon, percent, value }: { title: string, icon: any, percent: number, value?: string }) => {
    const getStatusColor = (p: number) => {
        if (p < 60) return 'text-primary';
        if (p < 80) return 'text-yellow-500';
        return 'text-red-500';
    };

    return (
        <div className="bg-card border border-border rounded-md p-2.5 flex flex-col gap-1">
            <div className="flex items-center justify-between mb-1">
                <span className={`${statsFontSizes.title} text-muted-foreground uppercase`}>{title}</span>
                <Icon className={`w-3 h-3 ${getStatusColor(percent)}`} />
            </div>
            <p className={`text-lg font-bold leading-none ${getStatusColor(percent)}`}>
                {percent.toFixed(1)}%
            </p>
            {value && (
                <p className="text-[8px] text-muted-foreground mt-0.5 truncate">
                    {value}
                </p>
            )}
            <div className="mt-1.5 h-1 bg-muted rounded-full overflow-hidden mt-auto">
                <div
                    className={`h-full ${getStatusColor(percent)} bg-current transition-all duration-300`}
                    style={{ width: `${percent}%` }}
                />
            </div>
        </div>
    );
};

const ManagementCard = ({
    title,
    badge,
    badgeColor,
    mainValue,
    subValue,
    children
}: {
    title: string;
    badge: string;
    badgeColor: string;
    mainValue: React.ReactNode;
    subValue: string;
    children?: React.ReactNode;
}) => {
    return (
        <div className="bg-card border border-border rounded-md p-2.5 cursor-pointer hover:bg-muted/10 transition-colors group">
            <div className="flex items-center justify-between mb-1">
                <span className={`${statsFontSizes.title} text-muted-foreground group-hover:text-foreground uppercase`}>{title}</span>
                <div className={`w-3 h-3 rounded-full flex items-center justify-center text-[8px] font-bold ${badgeColor}`}>
                    {badge}
                </div>
            </div>
            <div className="flex flex-col">
                <div className="flex items-baseline gap-1">
                    <span className="text-lg font-bold text-foreground leading-none">{mainValue}</span>
                </div>
                <span className="text-[9px] text-muted-foreground group-hover:text-primary transition-colors truncate">{subValue}</span>
            </div>
            <div className="mt-1.5">
                {children}
            </div>
        </div>
    );
};

export function SystemStats({ stats, cloudStats }: SystemStatsProps) {
    const formatBytes = (bytes: number) => {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
    };

    return (
        <div className="space-y-4">
            {/* Section 1: System Resources */}
            <div>
                <h3 className="text-xs font-semibold text-muted-foreground mb-2 uppercase tracking-wider">System Resources</h3>
                <div className="grid grid-cols-3 gap-2">
                    <ResourceCard
                        title="CPU"
                        icon={Cpu}
                        percent={stats.cpu_usage}
                    />
                    <ResourceCard
                        title="Memory"
                        icon={HardDrive}
                        percent={stats.memory_percent}
                        value={`${formatBytes(stats.memory_used)} / ${formatBytes(stats.memory_total)}`}
                    />
                    <ResourceCard
                        title="Storage"
                        icon={Database}
                        percent={stats.disk_percent}
                        value={`${formatBytes(stats.disk_used)} / ${formatBytes(stats.disk_total)}`}
                    />
                </div>
            </div>

            {/* Section 2: Management */}
            <div>
                <h3 className="text-xs font-semibold text-muted-foreground mb-2 uppercase tracking-wider">Management</h3>
                <div className="grid grid-cols-3 gap-2">
                    <ManagementCard
                        title="Apps"
                        badge="A"
                        badgeColor="bg-emerald-500/20 text-emerald-500"
                        mainValue={
                            <>
                                <span className="text-emerald-500 cursor-help" title="Active">{cloudStats.apps.active}</span>
                                <span className="text-muted-foreground">/</span>
                                <span className="text-muted-foreground cursor-help" title="Stopped">{cloudStats.apps.stopped}</span>
                                <span className="text-muted-foreground">/</span>
                                <span className="text-red-500 cursor-help" title="Crashed">{cloudStats.apps.crashed}</span>
                            </>
                        }
                        subValue="Applications"
                    />

                    <ManagementCard
                        title="Services"
                        badge="S"
                        badgeColor="bg-orange-500/20 text-orange-500"
                        mainValue={
                            <>
                                <span className="text-emerald-500 cursor-help" title="Active">{cloudStats.services.active}</span>
                                <span className="text-muted-foreground">/</span>
                                <span className="text-muted-foreground cursor-help" title="Stopped">{cloudStats.services.stopped}</span>
                                <span className="text-muted-foreground">/</span>
                                <span className="text-red-500 cursor-help" title="Crashed">{cloudStats.services.crashed}</span>
                            </>
                        }
                        subValue="DB & Tools"
                    />

                    <ManagementCard
                        title="Addons"
                        badge="P"
                        badgeColor="bg-purple-500/20 text-purple-500"
                        mainValue={
                            <div className="flex items-baseline gap-1">
                                <span className="text-emerald-500 cursor-help" title="Active">{cloudStats.addons.enabled}</span>
                                <span className="text-muted-foreground">/</span>
                                <span className="text-muted-foreground cursor-help" title="Stopped">{cloudStats.addons.installed}</span>
                            </div>
                        }
                        subValue="Plugins"
                    />
                </div>
            </div>
        </div >
    );
}
