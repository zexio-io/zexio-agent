import { Activity } from "lucide-react";

interface FooterProps {
    isOnline: boolean;
}

export function Footer({ isOnline }: FooterProps) {
    return (
        <div className="px-6 py-3 border-t border-zinc-800">
            <div className="flex items-center justify-between text-sm">
                <div className="flex items-center gap-2">
                    <Activity className={`w-4 h-4 ${isOnline ? 'text-emerald-500' : 'text-zinc-600'}`} />
                    <span className="text-zinc-500">Agent Status</span>
                </div>
                <div className="flex items-center gap-2">
                    <div className={`w-2 h-2 rounded-full ${isOnline ? 'bg-emerald-500' : 'bg-zinc-600'}`} />
                    <span className={isOnline ? 'text-emerald-500' : 'text-zinc-500'}>
                        {isOnline ? 'Online' : 'Offline'}
                    </span>
                </div>
            </div>
        </div>
    );
}
