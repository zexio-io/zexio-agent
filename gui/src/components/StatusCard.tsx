interface StatusCardProps {
    title: string;
    value: string | number;
    status?: "success" | "warning" | "error" | "neutral";
    icon?: string;
}

export function StatusCard({ title, value, status = "neutral", icon }: StatusCardProps) {
    const statusColors = {
        success: "text-green-400 border-green-500/20",
        warning: "text-yellow-400 border-yellow-500/20",
        error: "text-red-400 border-red-500/20",
        neutral: "text-gray-400 border-gray-700"
    };

    return (
        <div className={`p-6 bg-gray-800 rounded-xl border ${statusColors[status]} hover:border-blue-500/50 transition-colors`}>
            <div className="flex items-start justify-between">
                <div className="flex-1">
                    <h3 className="text-sm font-medium text-gray-400 mb-2">{title}</h3>
                    <p className="text-2xl font-bold text-white">{value}</p>
                </div>
                {icon && (
                    <div className="text-3xl opacity-50">{icon}</div>
                )}
            </div>
        </div>
    );
}
