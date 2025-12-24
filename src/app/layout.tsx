import type { Metadata, Viewport } from 'next';
import { Geist, Geist_Mono } from 'next/font/google';
import PWAInstallPrompt from '@/components/PWAInstallPrompt';
import NetworkStatus from '@/components/NetworkStatus';
import './globals.css';

const geistSans = Geist({
  variable: '--font-geist-sans',
  subsets: ['latin'],
});

const geistMono = Geist_Mono({
  variable: '--font-geist-mono',
  subsets: ['latin'],
});

export const metadata: Metadata = {
  title: 'Connect 4',
  description:
    'Play the classic Connect 4 strategy game. Drop your pieces to get 4 in a row and challenge an AI opponent in this timeless game.',
  keywords: [
    'board game',
    'classic game',
    'strategy game',
    'strategy',
    'AI',
    'Connect 4',
    'PWA',
    'offline game',
  ],
  authors: [{ name: 'Connect 4 Project' }],
  manifest: '/manifest.json',
  appleWebApp: {
    capable: true,
    statusBarStyle: 'black-translucent',
    title: 'Connect 4',
  },
  applicationName: 'Connect 4',
  other: {
    'mobile-web-app-capable': 'yes',
    'apple-mobile-web-app-capable': 'yes',
    'apple-mobile-web-app-status-bar-style': 'black-translucent',
    'apple-mobile-web-app-title': 'Connect 4',
    'msapplication-TileColor': '#000000',
    'msapplication-config': '/browserconfig.xml',
  },
};

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  themeColor: '#000000',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" href="/favicon.ico" sizes="any" />
        <link rel="apple-touch-icon" href="/icons/icon-192x192.png" />
        <link rel="mask-icon" href="/icons/icon-192x192.png" color="#1e40af" />

        <meta name="msapplication-TileImage" content="/icons/icon-144x144.png" />
        <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
        <meta name="theme-color" content="#000000" />
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />

        <script
          dangerouslySetInnerHTML={{
            __html: `
              if ('serviceWorker' in navigator) {
                window.addEventListener('load', function() {
                  navigator.serviceWorker.register('/sw.js')
                    .then(function(registration) {
                      registration.addEventListener('updatefound', function() {
                        const newWorker = registration.installing;
                        if (newWorker) {
                          newWorker.addEventListener('statechange', function() {
                            if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
                              if (document.getElementById('update-banner')) return;
                              const updateBanner = document.createElement('div');
                              updateBanner.id = 'update-banner';
                              updateBanner.innerHTML = "<div style='position:fixed;top:16px;left:50%;transform:translateX(-50%);background:rgba(30,41,59,0.97);color:#fff;padding:20px 24px 16px 24px;border-radius:18px;box-shadow:0 2px 16px rgba(0,0,0,0.18);z-index:10000;display:flex;flex-direction:column;align-items:center;gap:16px;font-family:system-ui;min-width:260px;max-width:90vw;'><span style='font-size:16px;font-weight:500;text-align:center;line-height:1.4;margin-bottom:8px;'>A new version is available!</span><div style='display:flex;gap:12px;width:100%;justify-content:center;'><button onclick='window.location.reload()' style='background:#38bdf8;color:#fff;border:none;border-radius:10px;padding:8px 20px;font-weight:600;cursor:pointer;font-size:15px;box-shadow:0 1px 4px rgba(56,189,248,0.15);transition:background 0.2s;' onmouseover='this.style.background=\\"#0ea5e9\\"' onmouseout='this.style.background=\\"#38bdf8\\"'>Update Now</button><button onclick='this.closest(\\"#update-banner\\").remove()' style='background:none;color:#fff;border:none;font-size:15px;text-decoration:underline;cursor:pointer;padding:8px 10px;'>Later</button></div></div>";
                              document.body.appendChild(updateBanner);
                            }
                          });
                        }
                      });
                      registration.update();
                    })
                    .catch(function(registrationError) {
                      console.log('SW registration failed: ', registrationError);
                    });
                });
              }
            `,
          }}
        />
        {process.env.NODE_ENV === 'development' && (
          <script
            dangerouslySetInnerHTML={{
              __html: `
                window.exposeGameStore = function(store) {
                  window.useGameStore = store;
                };
              `,
            }}
          />
        )}
      </head>
      <body className={`${geistSans.variable} ${geistMono.variable} antialiased`}>
        <div style={{ paddingTop: 'env(safe-area-inset-top)' }}>{children}</div>
        <PWAInstallPrompt />
        <NetworkStatus />
      </body>
    </html>
  );
}
