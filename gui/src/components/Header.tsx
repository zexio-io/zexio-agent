import { Settings } from "lucide-react";

interface HeaderProps {
    onSettingsClick: () => void;
}

export function Header({ onSettingsClick }: HeaderProps) {
    return (
        <div className="flex items-center justify-end px-6 py-4 border-b border-zinc-800">
            <button
                onClick={onSettingsClick}
                className="p-2 hover:bg-zinc-800 rounded-lg transition-colors"
            >
                <Settings className="w-5 h-5 text-zinc-400" />
            </button>
        </div>
    );
}
