// Capivara: guarda o site no aparelho para o app abrir rápido e funcionar sem internet.
// Sempre tenta a versão nova da internet primeiro; sem internet, usa a guardada.
const VERSAO = 'capivara-app-2';
const BASE = ['./', './index.html', './app.html', './en.html', './manifest.webmanifest', './icons/icon-192.png', './icons/icon-512.png'];

self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(VERSAO).then((c) => Promise.all(BASE.map((u) => c.add(u).catch(() => {})))));
  self.skipWaiting();
});

self.addEventListener('activate', (e) => {
  e.waitUntil(caches.keys()
    .then((ks) => Promise.all(ks.filter((k) => k.indexOf('capivara-app-') === 0 && k !== VERSAO).map((k) => caches.delete(k))))
    .then(() => self.clients.claim()));
});

self.addEventListener('fetch', (e) => {
  const r = e.request;
  if (r.method !== 'GET' || new URL(r.url).origin !== self.location.origin) return;   // Supabase, Stockfish etc. passam direto
  e.respondWith(fetch(r).then((resp) => {
    if (resp.ok) { const copia = resp.clone(); caches.open(VERSAO).then((c) => c.put(r, copia)); }
    return resp;
  }).catch(() => caches.match(r, { ignoreSearch: true }).then((m) => m || (r.mode === 'navigate' ? caches.match('./index.html') : Response.error()))));
});
