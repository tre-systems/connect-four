const CACHE_VERSION = '1766597744697-local-v1.0.0';
const CACHE_NAME = `connect-4-${CACHE_VERSION}`;
const OFFLINE_URL = '/offline';

const STATIC_ASSETS = [
  '/',
  '/offline',
  '/manifest.json',
  '/favicon.ico',
];

self.addEventListener('install', event => {
  console.log('[SW] Install event');

  event.waitUntil(
    caches
      .open(CACHE_NAME)
      .then(cache => {
        console.log('[SW] Caching static assets');
        return cache.addAll(STATIC_ASSETS);
      })
      .catch(error => {
        console.error('[SW] Failed to cache static assets:', error);
      })
  );

  self.skipWaiting();
});

self.addEventListener('activate', event => {
  console.log('[SW] Activate event');

  event.waitUntil(
    caches
      .keys()
      .then(cacheNames => {
        return Promise.all(
          cacheNames.map(cacheName => {
            if (cacheName !== CACHE_NAME) {
              console.log('[SW] Deleting old cache:', cacheName);
              return caches.delete(cacheName);
            }
          })
        );
      })
      .then(() => {
        console.log('[SW] Taking control of all pages');
        return self.clients.claim();
      })
  );
});

self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET') {
    return;
  }

  if (!event.request.url.startsWith('http')) {
    return;
  }

  const url = new URL(event.request.url);
  
  // WASM files - serve directly without caching
  if (url.pathname.startsWith('/wasm/')) {
    event.respondWith(fetch(event.request));
    return;
  }
  
  // Static assets (JS, CSS, images) - cache first
  if (url.pathname.startsWith('/_next/') || url.pathname.startsWith('/static/')) {
    event.respondWith(
      caches.match(event.request).then(cachedResponse => {
        if (cachedResponse) {
          return cachedResponse;
        }
        
        return fetch(event.request).then(response => {
          if (response && response.status === 200) {
            const responseToCache = response.clone();
            caches.open(CACHE_NAME).then(cache => {
              cache.put(event.request, responseToCache);
            });
          }
          return response;
        });
      })
    );
    return;
  }

  // For HTML pages and API calls - network first
  event.respondWith(
    fetch(event.request)
      .then(response => {
        if (!response || response.status !== 200 || response.type !== 'basic') {
          return response;
        }

        const responseToCache = response.clone();

        caches.open(CACHE_NAME).then(cache => {
          if (event.request.url.startsWith(self.location.origin)) {
            console.log('[SW] Caching new resource:', event.request.url);
            cache.put(event.request, responseToCache);
          }
        });

        return response;
      })
      .catch(error => {
        console.log('[SW] Fetch failed, trying cache:', error);
        
        return caches.match(event.request).then(cachedResponse => {
          if (cachedResponse) {
            return cachedResponse;
          }
          
          if (event.request.destination === 'document') {
            return caches.match(OFFLINE_URL);
          }
          
          throw error;
        });
      })
  );
});

self.addEventListener('sync', event => {
  console.log('[SW] Background sync event:', event.tag);

  if (event.tag === 'background-sync') {
    event.waitUntil(doBackgroundSync());
  }
});

async function doBackgroundSync() {
  console.log('[SW] Performing background sync');
}

self.addEventListener('push', event => {
  console.log('[SW] Push message received');

  const options = {
    body: event.data ? event.data.text() : 'Connect 4 notification',
    icon: '/icons/icon-192x192.png',
    badge: '/icons/icon-72x72.png',
    vibrate: [100, 50, 100],
    data: {
      dateOfArrival: Date.now(),
      primaryKey: 1,
    },
    actions: [
      {
        action: 'explore',
        title: 'Play Game',
        icon: '/icons/icon-72x72.png',
      },
      {
        action: 'close',
        title: 'Close',
        icon: '/icons/icon-72x72.png',
      },
    ],
  };

  event.waitUntil(self.registration.showNotification('Connect 4', options));
});

self.addEventListener('notificationclick', event => {
  console.log('[SW] Notification click received');

  event.notification.close();

  if (event.action === 'explore') {
    event.waitUntil(clients.openWindow('/'));
  }
});

self.addEventListener('message', event => {
  console.log('[SW] Message received:', event.data);
  
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }
});
