import { Settings } from "lucide-react";

interface HeaderProps {
    onSettingsClick: () => void;
}

export function Header({ onSettingsClick }: HeaderProps) {
    return (
        <div className="flex items-center justify-end px-6 py-4 border-b border-border">
            <button
                onClick={onSettingsClick}
                className="p-2 hover:bg-border rounded-lg transition-colors"
            >
                <Settings className="w-5 h-5 text-muted-foreground" />
            </button>
        </div>
    );
}
