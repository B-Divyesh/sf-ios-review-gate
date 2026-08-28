const CACHE = 'ios-review-gate-v2';
const SHELL = ['/', '/demo', '/privacy', '/terms', '/assets/b612-mono.woff2', '/assets/release-blueprint.webp', '/assets/terminal-recording.svg'];

async function appAssets() {
  const home = await fetch('/', { cache: 'no-store' });
  const html = await home.text();
  return [...html.matchAll(/(?:href|src)="([^"]+)"/g)]
    .map(([, asset]) => new URL(asset, self.location.origin).pathname)
    .filter(asset => asset.startsWith('/assets/'));
}

self.addEventListener('install', event => {
  event.waitUntil(caches.open(CACHE).then(async cache => {
    await cache.addAll([...SHELL, ...await appAssets()]);
    await self.skipWaiting();
  }));
});

self.addEventListener('activate', event => {
  event.waitUntil(caches.keys()
    .then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key))))
    .then(() => self.clients.claim()));
});

self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET' || new URL(event.request.url).origin !== self.location.origin) return;
  event.respondWith(caches.open(CACHE)
    .then(cache => cache.match(event.request, { ignoreVary: true }))
    .then(cached => cached || fetch(event.request)));
});
