import { Activity } from "lucide-react";

interface FooterProps {
    isOnline: boolean;
}

export function Footer({ isOnline }: FooterProps) {
    return (
        <div className="px-6 py-3 border-t border-border">
            <div className="flex items-center justify-between text-sm">
                <div className="flex items-center gap-2">
                    <Activity className={`w-4 h-4 ${isOnline ? 'text-primary' : 'text-muted-foreground'}`} />
                    <span className="text-muted-foreground">Agent Status</span>
                </div>
                <div className="flex items-center gap-2">
                    <div className={`w-2 h-2 rounded-full ${isOnline ? 'bg-primary' : 'bg-muted-foreground'}`} />
                    <span className={isOnline ? 'text-primary' : 'text-muted-foreground'}>
                        {isOnline ? 'Online' : 'Offline'}
                    </span>
                </div>
            </div>
        </div>
    );
}
