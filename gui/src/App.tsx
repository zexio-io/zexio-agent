import { useState } from "react";
import "./App.css";

function App() {
  return (
    <div className="min-h-screen bg-gray-900 text-white font-sans">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 bg-gray-800 border-b border-gray-700 shadow-md">
        <div className="flex items-center gap-3">
          <img src="/logo.png" alt="Zexio Logo" className="w-10 h-10 object-contain" />
          <div>
            <h1 className="text-xl font-bold tracking-tight text-white">Zexio Agent</h1>
            <p className="text-xs text-gray-400">Local Dashboard</p>
          </div>
        </div>
        <div className="flex items-center gap-4">
          <div className="px-3 py-1 text-xs font-medium text-green-400 bg-green-400/10 rounded-full border border-green-400/20">
            ● Online
          </div>
        </div>
      </header>

      {/* Main Content Placeholder */}
      <main className="p-8 max-w-5xl mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {/* Status Card */}
          <div className="p-6 bg-gray-800 rounded-xl border border-gray-700 hover:border-blue-500/50 transition-colors">
            <h3 className="text-sm font-medium text-gray-400 mb-2">Tunnel Status</h3>
            <div className="flex items-center justify-between">
              <span className="text-2xl font-bold text-white">Inactive</span>
              <button className="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold rounded-lg transition-colors">
                Start Tunnel
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
