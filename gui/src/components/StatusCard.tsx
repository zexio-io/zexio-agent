import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { LucideIcon } from "lucide-react";

interface StatusCardProps {
    title: string;
    value: string | number;
    status?: "success" | "warning" | "error" | "neutral";
    icon?: LucideIcon;
}

export function StatusCard({ title, value, status = "neutral", icon: Icon }: StatusCardProps) {
    const statusColors = {
        success: "text-green-400",
        warning: "text-yellow-400",
        error: "text-red-400",
        neutral: "text-muted-foreground"
    };

    return (
        <Card>
            <CardHeader className="pb-2 pt-4 px-4">
                <div className="flex items-center justify-between">
                    <CardTitle className="text-xs font-medium text-muted-foreground">{title}</CardTitle>
                    {Icon && <Icon className={`h-4 w-4 ${statusColors[status]}`} />}
                </div>
            </CardHeader>
            <CardContent className="px-4 pb-4">
                <p className={`text-xl font-bold ${statusColors[status]}`}>{value}</p>
            </CardContent>
        </Card>
    );
}
