'use client';

import { useState, useEffect } from 'react';
import { Wifi, WifiOff } from 'lucide-react';

export default function NetworkStatus() {
  const [isOnline, setIsOnline] = useState(true);
  const [showStatus, setShowStatus] = useState(false);

  useEffect(() => {
    const updateOnlineStatus = () => {
      setIsOnline(navigator.onLine);
      setShowStatus(true);
      setTimeout(() => setShowStatus(false), 3000);
    };

    updateOnlineStatus();

    window.addEventListener('online', updateOnlineStatus);
    window.addEventListener('offline', updateOnlineStatus);

    return () => {
      window.removeEventListener('online', updateOnlineStatus);
      window.removeEventListener('offline', updateOnlineStatus);
    };
  }, []);

  if (!showStatus) return null;

  return (
    <div className="fixed top-4 left-4 z-50 p-2 rounded-full shadow-lg transition-all duration-300 bg-white/10 backdrop-blur-sm flex items-center justify-center">
      {isOnline ? (
        <Wifi className="h-5 w-5 text-green-400" />
      ) : (
        <WifiOff className="h-5 w-5 text-red-400" />
      )}
    </div>
  );
}
