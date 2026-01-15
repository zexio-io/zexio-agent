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
        neutral: "text-gray-400"
    };

    const cardVariant = status === "neutral" ? "default" : status;

    return (
        <Card variant={cardVariant as any}>
            <CardHeader className="pb-3">
                <div className="flex items-center justify-between">
                    <CardTitle className="text-sm font-medium text-gray-400">{title}</CardTitle>
                    {Icon && <Icon className={`h-5 w-5 ${statusColors[status]}`} />}
                </div>
            </CardHeader>
            <CardContent>
                <p className={`text-2xl font-bold ${statusColors[status]}`}>{value}</p>
            </CardContent>
        </Card>
    );
}
