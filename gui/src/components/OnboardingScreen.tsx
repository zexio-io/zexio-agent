import { useState } from "react";
import { ModeSelector } from "./ModeSelector";
import { CloudAuthForm } from "./CloudAuthForm";
import { StandaloneConfig } from "./StandaloneConfig";
import { LogoBrand } from "./LogoBrand";

type OnboardingStep = "mode-select" | "cloud-auth" | "standalone-config";

interface OnboardingScreenProps {
    onComplete: (config: {
        mode: "cloud" | "standalone";
        token?: string;
        workerId?: string;
        apiPort?: number;
        meshPort?: number;
    }) => void;
}

export function OnboardingScreen({ onComplete }: OnboardingScreenProps) {
    const [step, setStep] = useState<OnboardingStep>("mode-select");

    const handleCloudSubmit = (token: string, workerId: string) => {
        onComplete({
            mode: "cloud",
            token,
            workerId,
        });
    };

    const handleStandaloneSubmit = (apiPort: number, meshPort: number) => {
        onComplete({
            mode: "standalone",
            apiPort,
            meshPort,
        });
    };

    return (
        <div className="h-screen bg-zinc-950 text-white flex flex-col items-center justify-center px-6">
            <div className="mb-12">
                <LogoBrand size="lg" />
            </div>

            {step === "mode-select" && (
                <ModeSelector
                    onSelectCloud={() => setStep("cloud-auth")}
                    onSelectStandalone={() => setStep("standalone-config")}
                />
            )}

            {step === "cloud-auth" && (
                <CloudAuthForm
                    onSubmit={handleCloudSubmit}
                    onBack={() => setStep("mode-select")}
                />
            )}

            {step === "standalone-config" && (
                <StandaloneConfig
                    onSubmit={handleStandaloneSubmit}
                    onBack={() => setStep("mode-select")}
                />
            )}
        </div>
    );
}
